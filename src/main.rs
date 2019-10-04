extern crate chrono;
extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate termion;
#[macro_use]
extern crate cached;

pub mod style;
pub mod widgets;

use style::*;

use clap::{App, Arg};
use std::path::PathBuf;
use std::string::String;

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
        0 => green("✓"),
        _ => red("✗"),
    };

    let job_symbol = match widgets::shell::get_shell_jobs(args.value_of("JOBS").unwrap()).len() {
        0 => format!(""),
        _ => cyan("⚙"),
    };

    let (width, height) = widgets::terminal::get_terminal_size();

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

    let cwd_text = bold(red(cwd.display()));

    let mut git_text = String::new();
    match widgets::git::get_git_repo(widgets::shell::get_cwd()) {
        None => (),
        Some(repo) => {
            let status = widgets::git::handle_git_repo(&repo);
            git_text = green(format!("(git:{})", status.description));
        }
    }

    let prompt_symbol = match widgets::user::is_root() {
        true => bold(red("#")),
        false => bold(green("$")),
    };

    println!("{}", yellow(format!("{} {} {}", shell, line, time)));
    println!(
        "{} {} {}{} {} {}",
        retval_symbol,
        job_symbol,
        green(format!("{}@{}:", passwd.username, hostname)),
        cwd_text,
        git_text,
        prompt_symbol
    );
}
