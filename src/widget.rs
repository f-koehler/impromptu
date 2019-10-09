use std::collections::LinkedList;
use std::string::String;

use super::style;

enum SizePolicy {
    Fixed,
    Expanding,
}

struct Widget {
    size_policy: SizePolicy,
    content: String,
    foreground: Option<style::Color>,
}

impl Widget {
    fn new() -> Widget {
        Widget {
            size_policy: SizePolicy::Fixed,
            content: String::new(),
            foreground: None,
        }
    }

    fn text(text: String) -> Widget {
        Widget {
            size_policy: SizePolicy::Fixed,
            content: text,
            foreground: None,
        }
    }

    fn space(symbol: String) -> Widget {
        Widget {
            size_policy: SizePolicy::Expanding,
            content: symbol,
            foreground: None,
        }
    }

    fn size(&self) -> usize {
        self.content.len()
    }
}

struct Line {
    widgets: LinkedList<Widget>,
}

impl Line {
    fn add(&mut self, widget: Widget) {
        self.widgets.push_back(widget);
    }

    fn add_with_separator(&mut self, widget: Widget, separator: String) {
        match self.widgets.back() {
            None => self.widgets.push_back(widget),
            Some(_last) => {
                self.widgets.push_back(Widget::text(separator));
                self.widgets.push_back(widget);
            }
        }
    }
}
