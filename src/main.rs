extern crate chrono;
extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate nix;
extern crate termion;
#[macro_use]
extern crate cached;

pub mod git;
pub mod hostname;
pub mod python;
pub mod shell;
pub mod style;
pub mod terminal;
pub mod user;

use style::*;

use clap::{App, Arg};
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

    let job_symbol = match shell::get_shell_jobs(args.value_of("JOBS").unwrap()).len() {
        0 => format!(""),
        _ => cyan("⚙ "),
    };

    let (width, _) = terminal::get_terminal_size();

    let time = format!("{}", chrono::Local::now().format("%H:%M:%S"));
    let shell = shell::get_shell_name();
    let line = match width {
        0u16 => "".to_string(),
        _ => std::iter::repeat("─")
            .take(width as usize - time.len() - shell.len() - 1)
            .collect::<String>(),
    };

    let hostname = hostname::get_hostname();

    let passwd = user::get_passwd();

    let cwd_text = bold(red(shell::shorten_path(
        shell::get_cwd(),
        passwd.home_directory,
    )
    .display()));

    let mut git_text = String::new();
    match git::get_git_repo(shell::get_cwd()) {
        None => (),
        Some(repo) => {
            let status = git::handle_git_repo(&repo);

            if status.number_new > 0 {
                git_text = format!("{}{}", git_text, green(format!(" ★{}", status.number_new)));
            }

            if status.number_deleted > 0 {
                git_text = format!(
                    "{}{}",
                    git_text,
                    red(format!(" 🗑{}", status.number_deleted))
                );
            }

            if status.number_modified > 0 {
                git_text = format!(
                    "{}{}",
                    git_text,
                    yellow(format!(" 📝{}", status.number_modified))
                );
            }

            if status.number_conflicts > 0 {
                git_text = format!(
                    "{}{}",
                    git_text,
                    cyan(format!(" ⚡{}", status.number_conflicts))
                );
            }

            git_text = format!(
                "{}{}{}",
                green(format!("(git:{}", status.description)),
                git_text,
                green(")")
            );
        }
    }

    let prompt_symbol = match user::is_root() {
        true => bold(red("#")),
        false => bold(green("$")),
    };

    let python_text = match python::get_virtual_env_name() {
        None => String::new(),
        Some(venv) => format!("[py: {}] ", venv),
    };

    println!("{}", yellow(format!("{} {} {}", shell, line, time)));
    print!(
        "{} {}{}{}{} {} {}",
        retval_symbol,
        job_symbol,
        python_text,
        green(format!("{}@{}:", passwd.username, hostname)),
        cwd_text,
        git_text,
        prompt_symbol
    );
}
