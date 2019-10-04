extern crate chrono;
extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate termion;

use clap::{App, Arg};
use regex::Regex;
use std::ffi::{CStr, CString};
use std::io::Read;
use std::path::PathBuf;
use std::string::String;
use termion::{color, style};

struct ShellJob {
    job_number: u32,
    is_current: bool,
    pid: u32,
    state: String,
    command: String,
}

impl ShellJob {
    fn from_captures(captures: &regex::Captures) -> ShellJob {
        ShellJob {
            job_number: match captures[0].parse::<u32>() {
                Err(_err) => 0,
                Ok(number) => number,
            },
            is_current: match &captures[1] {
                "+" => true,
                _ => false,
            },
            pid: match captures[2].parse::<u32>() {
                Err(_err) => 0,
                Ok(pid) => pid,
            },
            state: captures[3].to_string(),
            command: captures[4].to_string(),
        }
    }
}

fn get_shell_jobs(arg: &str) -> Vec<ShellJob> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[(\d+)\]\s*([+-])\s*(\d+)\s*(\w+)\s*(.+)$").unwrap();
    }

    let mut result: Vec<ShellJob> = Vec::new();

    for line in arg.lines() {
        match RE.captures(line) {
            None => (),
            Some(captures) => (result.push(ShellJob::from_captures(&captures))),
        }
    }

    result
}

struct WinSize {
    ws_row: nix::libc::c_ushort,
    ws_col: nix::libc::c_ushort,
    ws_xpixel: nix::libc::c_ushort,
    ws_ypixel: nix::libc::c_ushort,
}

fn get_terminal_size() -> (u16, u16) {
    unsafe {
        let mut winsize = WinSize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        match nix::libc::ioctl(
            nix::libc::STDOUT_FILENO,
            nix::libc::TIOCGWINSZ,
            &mut winsize,
        ) {
            -1 => return (0u16, 0u16),
            _ => {
                return (
                    std::cmp::max(0, winsize.ws_col) as u16,
                    std::cmp::max(0, winsize.ws_row),
                )
            }
        }
    }
}

fn get_shell_name() -> String {
    let pid = nix::unistd::getppid().as_raw();
    match std::fs::File::open(format!("/proc/{}/cmdline", pid)) {
        Err(_err) => "unknown".to_string(),
        Ok(mut file) => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Err(_err) => "unknown".to_string(),
                Ok(_ok) => String::from(buf),
            }
        }
    }
}

struct Passwd {
    username: String,
    home_directory: PathBuf,
}

fn get_passwd() -> Passwd {
    let mut result = Passwd {
        username: String::new(),
        home_directory: PathBuf::new(),
    };
    unsafe {
        let passwd = nix::libc::getpwuid(nix::libc::geteuid());
        result.username = CStr::from_ptr((*passwd).pw_name)
            .to_string_lossy()
            .into_owned();
        result.home_directory = PathBuf::from(
            CStr::from_ptr((*passwd).pw_dir)
                .to_string_lossy()
                .into_owned(),
        );
    }
    result
}

fn is_root() -> bool {
    nix::unistd::geteuid().is_root()
}

fn get_hostname() -> String {
    let buffer_size = match nix::unistd::sysconf(nix::unistd::SysconfVar::HOST_NAME_MAX).unwrap() {
        Some(len) => len as usize,
        None => 128 as usize,
    };
    let mut buf: Vec<u8> = Vec::with_capacity(buffer_size);
    buf.resize(buffer_size, 0u8);
    match nix::unistd::gethostname(buf.as_mut()) {
        Err(_err) => "unknown".to_string(),
        Ok(hostname) => hostname.to_string_lossy().into_owned(),
    }
}

fn get_git_repo(path: PathBuf) -> Option<git2::Repository> {
    let result = git2::Repository::discover(path);
    if result.is_ok() {
        result.ok()
    } else {
        None
    }
}

struct GitStatus {
    has_modifications: bool,
    is_detached: bool,
    is_clean: bool,
    state: git2::RepositoryState,
    description: String,
}

fn handle_git_repo(repo: &git2::Repository) -> GitStatus {
    let mut result = GitStatus {
        has_modifications: false,
        is_detached: false,
        is_clean: false,
        state: git2::RepositoryState::Clean,
        description: String::new(),
    };

    match repo.head_detached() {
        Err(_err) => (),
        Ok(detached) => result.is_detached = detached,
    }

    match repo.statuses(None) {
        Err(_err) => (),
        Ok(statuses) => {
            for i in 0..statuses.len() {
                match statuses.get(i) {
                    None => (),
                    Some(status) => result.has_modifications |= status.status().bits() != 0,
                }
            }
        }
    }

    let mut describe_options = git2::DescribeOptions::new();
    describe_options.describe_all();
    match repo.describe(&describe_options) {
        Err(_err) => result.description = "Ø".to_string(),
        Ok(description) => match description.format(None) {
            Err(_err) => result.description = "unknw".to_string(),
            Ok(text) => result.description = text,
        },
    }

    result.is_clean = repo.state() == git2::RepositoryState::Clean;

    result
}

fn main() {
    let args = App::new("impromptu")
        .version("0.1")
        .author("Fabian Köhler <fkoehler1024@googlemail.com>")
        .about("A portable and fast shell for your prompt")
        .arg(
            Arg::with_name("RETVAL")
                .help("Return code of last shell command")
                .required(true),
        )
        .arg(
            Arg::with_name("JOBS")
                .help("Jobs of this shell")
                .required(true),
        )
        .get_matches();

    let retval = match args.value_of("RETVAL").unwrap().parse::<i64>() {
        Err(_err) => 1,
        Ok(retval) => retval,
    };
    let retval_symbol = match retval {
        0 => format!("{}✓{}", color::Fg(color::Green), color::Fg(color::Reset)),
        _ => format!("{}✗{}", color::Fg(color::Red), color::Fg(color::Reset)),
    };

    let job_symbol = match get_shell_jobs(args.value_of("JOBS").unwrap()).len() {
        0 => format!(""),
        _ => format!(" {}⚙{}", color::Fg(color::Cyan), color::Fg(color::Reset)),
    };

    let (width, height) = get_terminal_size();

    let time = format!("{}", chrono::Local::now().format("%H:%M:%S"));
    let shell = get_shell_name();
    let line = std::iter::repeat("─")
        .take(width as usize - time.len() - shell.len() - 1)
        .collect::<String>();

    let hostname = get_hostname();

    let passwd = get_passwd();
    let cwd = match std::env::current_dir() {
        Err(_err) => PathBuf::from("unknown"),
        Ok(path) => match path.strip_prefix(passwd.home_directory) {
            Err(_err) => path,
            Ok(stripped) => PathBuf::from("~").join(stripped),
        },
    };

    let cwd_text = format!(
        "{}{}{}{}{}",
        style::Bold,
        color::Fg(color::Red),
        cwd.display(),
        color::Fg(color::Reset),
        style::Reset
    );

    let mut git_text = String::new();
    match get_git_repo(std::env::current_dir().ok().unwrap()) {
        None => (),
        Some(repo) => {
            let status = handle_git_repo(&repo);
            git_text = format!(
                "{}(git:{}){}",
                color::Fg(color::Green),
                status.description,
                color::Fg(color::Reset)
            )
            .to_string();
        }
    }

    let prompt_symbol = match is_root() {
        true => format!(
            "{}{}#{}{}",
            style::Bold,
            color::Fg(color::Red),
            color::Fg(color::Reset),
            style::Reset
        ),
        false => format!(
            "{}{}${}{}",
            style::Bold,
            color::Fg(color::Green),
            color::Fg(color::Reset),
            style::Reset
        ),
    };

    println!(
        "{}{} {} {}{}",
        color::Fg(color::Yellow),
        shell,
        line,
        time,
        color::Fg(color::Reset)
    );
    println!(
        "{}{} {}{}@{}{}:{} {} {}",
        retval_symbol,
        job_symbol,
        color::Fg(color::Green),
        passwd.username,
        color::Fg(color::Green),
        hostname,
        cwd_text,
        git_text,
        prompt_symbol
    );
}
