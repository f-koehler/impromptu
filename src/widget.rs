use std::collections::LinkedList;
use std::string::String;

use super::style::{bg_colored, colored, styled, Color, Style};

pub enum SizePolicy {
    Fixed,
    Expanding,
}

pub struct Widget {
    size_policy: SizePolicy,
    content: String,
    foreground: Option<Color>,
    background: Option<Color>,
    style: Option<Style>,
}

impl Widget {
    pub fn new() -> Self {
        Widget {
            size_policy: SizePolicy::Fixed,
            content: String::new(),
            foreground: None,
            background: None,
            style: None,
        }
    }

    pub fn text<T: std::fmt::Display>(text: T) -> Self {
        Widget {
            size_policy: SizePolicy::Fixed,
            content: format!("{}", text),
            foreground: None,
            background: None,
            style: None,
        }
    }

    pub fn spacer(symbol: String) -> Self {
        Widget {
            size_policy: SizePolicy::Expanding,
            content: symbol,
            foreground: None,
            background: None,
            style: None,
        }
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }

    pub fn set_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }
    pub fn set_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }
    pub fn set_style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn to_string(&self) -> String {
        match self.foreground {
            None => match self.background {
                None => match self.style {
                    None => self.content.to_owned(),
                    Some(style) => styled(self.content.to_owned(), style),
                },
                Some(bg) => match self.style {
                    None => bg_colored(self.content.to_owned(), bg),
                    Some(style) => bg_colored(styled(self.content.to_owned(), style), bg),
                },
            },
            Some(fg) => match self.background {
                None => match self.style {
                    None => colored(self.content.to_owned(), fg),
                    Some(style) => colored(styled(self.content.to_owned(), style), fg),
                },
                Some(bg) => match self.style {
                    None => colored(bg_colored(self.content.to_owned(), bg), fg),
                    Some(style) => {
                        colored(bg_colored(styled(self.content.to_owned(), style), bg), fg)
                    }
                },
            },
        }
    }
}

struct Line {
    widgets: LinkedList<Widget>,
}

impl Line {
    pub fn add(&mut self, widget: Widget) {
        self.widgets.push_back(widget);
    }

    pub fn add_with_separator(&mut self, widget: Widget, separator: String) {
        match self.widgets.back() {
            None => self.widgets.push_back(widget),
            Some(last) => {
                let mut separator = Widget::text(separator);

                match last.background {
                    None => (),
                    Some(bg) => separator.foreground = Some(bg),
                };

                match widget.foreground {
                    None => (),
                    Some(fg) => separator.background = Some(fg),
                };

                self.widgets.push_back(separator);
                self.widgets.push_back(widget);
            }
        }
    }

    pub fn add_with_separator_reversed(&mut self, widget: Widget, separator: String) {
        match self.widgets.back() {
            None => self.widgets.push_back(widget),
            Some(last) => {
                let mut separator = Widget::text(separator);

                match last.foreground {
                    None => (),
                    Some(fg) => separator.background = Some(fg),
                };

                match widget.background {
                    None => (),
                    Some(bg) => separator.foreground = Some(bg),
                };

                self.widgets.push_back(separator);
                self.widgets.push_back(widget);
            }
        }
    }
}
