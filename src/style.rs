pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
}

pub enum Style {
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,
    Invert,
    CrossedOut,
    Framed,
}

pub fn colored<T: std::fmt::Display>(arg: T, color: Color) -> String {
    match color {
        Color::Black => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Black),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::Red => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Red),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::Green => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Green),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::Yellow => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Yellow),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::Blue => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Blue),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::Magenta => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Magenta),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::Cyan => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::Cyan),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::White => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::White),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightBlack => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightBlack),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightRed => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightRed),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightGreen => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightGreen),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightYellow => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightYellow),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightBlue => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightBlue),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightMagenta => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightMagenta),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightCyan => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightCyan),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
        Color::LightWhite => format!(
            "{}{}{}",
            termion::color::Fg(termion::color::LightWhite),
            arg,
            termion::color::Fg(termion::color::Reset)
        ),
    }
}

pub fn bg_colored<T: std::fmt::Display>(arg: T, color: Color) -> String {
    match color {
        Color::Black => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Black),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::Red => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Red),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::Green => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Green),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::Yellow => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Yellow),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::Blue => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Blue),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::Magenta => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Magenta),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::Cyan => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::Cyan),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::White => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::White),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightBlack => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightBlack),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightRed => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightRed),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightGreen => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightGreen),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightYellow => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightYellow),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightBlue => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightBlue),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightMagenta => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightMagenta),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightCyan => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightCyan),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
        Color::LightWhite => format!(
            "{}{}{}",
            termion::color::Bg(termion::color::LightWhite),
            arg,
            termion::color::Bg(termion::color::Reset)
        ),
    }
}

pub fn styled<T: std::fmt::Display>(arg: T, style: Style) -> String {
    match style {
        Style::Bold => format!("{}{}{}", termion::style::Bold, arg, termion::style::Reset),
        Style::Faint => format!("{}{}{}", termion::style::Faint, arg, termion::style::Reset),
        Style::Italic => format!("{}{}{}", termion::style::Italic, arg, termion::style::Reset),
        Style::Underline => format!(
            "{}{}{}",
            termion::style::Underline,
            arg,
            termion::style::Reset
        ),
        Style::Blink => format!("{}{}{}", termion::style::Blink, arg, termion::style::Reset),
        Style::Invert => format!("{}{}{}", termion::style::Invert, arg, termion::style::Reset),
        Style::CrossedOut => format!(
            "{}{}{}",
            termion::style::CrossedOut,
            arg,
            termion::style::Reset
        ),
        Style::Framed => format!("{}{}{}", termion::style::Framed, arg, termion::style::Reset),
    }
}
