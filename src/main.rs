extern crate chrono;
extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate nix;
#[macro_use]
extern crate cached;

pub mod git;
pub mod hostname;
pub mod python;
pub mod shell;
pub mod style;
pub mod terminal;
pub mod user;
pub mod widget;

use style::Color::*;
use style::Style::*;
use style::*;
use widget::Widget;

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

    let passwd = user::get_passwd();

    let mut line1 = std::vec::Vec::new();
    line1.push(Widget::text(shell::get_shell_name()).set_foreground(Yellow));
    line1.push(Widget::spacer("─".to_string()).set_foreground(Yellow));
    line1.push(Widget::text(chrono::Local::now().format("%H:%M:%S")).set_foreground(Yellow));

    let mut line2 = std::vec::Vec::new();
    line2.push(match shell::parse_retval(&args) {
        0 => Widget::text("✓").set_foreground(Green),
        _ => Widget::text("✗").set_foreground(Red),
    });
    line2.push(
        match shell::get_shell_jobs(args.value_of("JOBS").unwrap()).len() {
            0 => Widget::text(""),
            _ => Widget::text("⚙ ").set_foreground(Cyan),
        },
    );
    line2.push(match python::get_virtual_env_name() {
        None => Widget::new(),
        Some(venv) => Widget::text(format!("[py: {}] ", venv)),
    });
    line2.push(Widget::text(passwd.username).set_foreground(Green));
    line2.push(Widget::text("@".to_string()).set_foreground(Green));
    line2.push(Widget::text(hostname::get_hostname()).set_foreground(Green));
    line2.push(Widget::text(":").set_foreground(Green));
    line2.push(
        Widget::text(shell::shorten_path(shell::get_cwd(), passwd.home_directory).display())
            .set_foreground(Red)
            .set_style(Bold),
    );

    let (width, _) = terminal::get_terminal_size();

    // let line = match width {
    //     0u16 => "".to_string(),
    //     _ => std::iter::repeat("─")
    //         .take(width as usize - time.len() - shell.len() - 1)
    //         .collect::<String>(),
    // };

    let mut git_text = String::new();
    match git::get_git_repo(shell::get_cwd()) {
        None => (),
        Some(repo) => {
            let status = git::handle_git_repo(&repo);

            if status.number_new > 0 {
                git_text = format!(
                    "{}{}",
                    git_text,
                    colored(format!(" ★{}", status.number_new), Green)
                );
            }

            if status.number_deleted > 0 {
                git_text = format!(
                    "{}{}",
                    git_text,
                    colored(format!(" 🗑{}", status.number_deleted), Red)
                );
            }

            if status.number_modified > 0 {
                git_text = format!(
                    "{}{}",
                    git_text,
                    colored(format!(" 📝{}", status.number_modified), Yellow)
                );
            }

            if status.number_conflicts > 0 {
                git_text = format!(
                    "{}{}",
                    git_text,
                    colored(format!(" ⚡{}", status.number_conflicts), Cyan)
                );
            }

            git_text = format!(
                "{}{}{}",
                colored(format!("(git:{}", status.description), Green),
                git_text,
                colored(")", Green)
            );
        }
    }

    let widget_symbol = match user::is_root() {
        true => Widget::text("#").set_foreground(Red),
        false => Widget::text("$").set_foreground(Green),
    };

    for widget in line1 {
        print!("{}", widget.to_string());
    }
    println!("");
    for widget in line2 {
        print!("{}", widget.to_string());
    }

    // println!(
    //     "{}",
    //     colored(format!("{} {} {}", shell, line, time), Yellow)
    // );
    // print!(
    //     "{} {}{}{}{} {} {}",
    //     widget_retval.to_string(),
    //     widget_jobs.to_string(),
    //     python_text,
    //     colored(format!("{}@{}:", passwd.username, hostname), Green),
    //     cwd_text,
    //     git_text,
    //     widget_symbol.to_string()
    // );
}
