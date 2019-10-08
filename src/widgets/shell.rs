extern crate regex;

use self::regex::{Captures, Regex};
use std::io::Read;
use std::path::PathBuf;

pub struct ShellJob {
    pub job_number: u32,
    pub is_current: bool,
    pub pid: u32,
    pub state: String,
    pub command: String,
}

impl ShellJob {
    fn from_captures(captures: &Captures) -> ShellJob {
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

pub fn get_shell_jobs(arg: &str) -> Vec<ShellJob> {
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

cached! {
    SHELL_NAME;
    fn get_shell_name() -> String = {
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
}

cached! {
    CWD;
    fn get_cwd() -> PathBuf = {
        match std::env::current_dir() {
            Err(_err) => PathBuf::from("<unknown>"),
            Ok(path) => path,
        }
    }
}

pub fn shorten_path(path: PathBuf, home_path: PathBuf) -> PathBuf {
    match path.strip_prefix(home_path) {
        Err(_) => path,
        Ok(stripped) => match stripped.to_str() {
            None => path,
            Some(converted) => match converted.trim().is_empty() {
                true => PathBuf::from("~"),
                false => PathBuf::from("~").join(stripped),
            },
        },
    }
}
