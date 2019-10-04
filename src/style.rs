pub fn colored<T: std::fmt::Display, C: termion::color::Color>(arg: T, col: C) -> String {
    format!(
        "{}{}{}",
        termion::color::Fg(col),
        arg,
        termion::color::Fg(termion::color::Reset)
    )
}

pub fn black<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::Black)
}

pub fn blue<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::Blue)
}

pub fn cyan<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::Cyan)
}

pub fn green<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::Green)
}

pub fn light_black<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightBlack)
}

pub fn light_blue<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightBlue)
}

pub fn light_cyan<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightCyan)
}

pub fn light_green<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightGreen)
}

pub fn light_magenta<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightMagenta)
}

pub fn light_red<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightRed)
}

pub fn light_white<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightWhite)
}

pub fn light_yellow<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::LightYellow)
}

pub fn magenta<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::Magenta)
}

pub fn red<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::Red)
}

pub fn white<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::White)
}

pub fn yellow<T: std::fmt::Display>(arg: T) -> String {
    colored(arg, termion::color::Yellow)
}

pub fn styled<T: std::fmt::Display, S: std::fmt::Display>(arg: T, style: S) -> String {
    format!("{}{}{}", style, arg, termion::style::Reset)
}

pub fn bold<T: std::fmt::Display>(arg: T) -> String {
    styled(arg, termion::style::Bold)
}

pub fn italic<T: std::fmt::Display>(arg: T) -> String {
    styled(arg, termion::style::Italic)
}

pub fn underline<T: std::fmt::Display>(arg: T) -> String {
    styled(arg, termion::style::Underline)
}
