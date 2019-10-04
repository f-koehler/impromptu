extern crate chrono;
extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate termion;
#[macro_use]
extern crate cached;

pub mod widgets;

use clap::{App, Arg};
use std::path::PathBuf;
use std::string::String;
use termion::{color, style};

struct TerminalSize {
    ws_row: nix::libc::c_ushort,
    ws_col: nix::libc::c_ushort,
    ws_xpixel: nix::libc::c_ushort,
    ws_ypixel: nix::libc::c_ushort,
}

cached! {
    TERMINAL_SIZE;

    fn get_terminal_size() -> (u16, u16) = {
        unsafe {
            let mut winsize = TerminalSize {
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

    let job_symbol = match widgets::shell::get_shell_jobs(args.value_of("JOBS").unwrap()).len() {
        0 => format!(""),
        _ => format!(" {}⚙{}", color::Fg(color::Cyan), color::Fg(color::Reset)),
    };

    let (width, height) = get_terminal_size();

    let time = format!("{}", chrono::Local::now().format("%H:%M:%S"));
    let shell = widgets::shell::get_shell_name();
    let line = std::iter::repeat("─")
        .take(width as usize - time.len() - shell.len() - 1)
        .collect::<String>();

    let hostname = widgets::hostname::get_hostname();

    let passwd = widgets::user::get_passwd();
    let cwd = match widgets::shell::get_cwd().strip_prefix(passwd.home_directory) {
        Err(_err) => widgets::shell::get_cwd(),
        Ok(stripped) => PathBuf::from("~").join(stripped),
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
    match widgets::git::get_git_repo(widgets::shell::get_cwd()) {
        None => (),
        Some(repo) => {
            let status = widgets::git::handle_git_repo(&repo);
            git_text = format!(
                "{}(git:{}){}",
                color::Fg(color::Green),
                status.description,
                color::Fg(color::Reset)
            )
            .to_string();
        }
    }

    let prompt_symbol = match widgets::user::is_root() {
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
