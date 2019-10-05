mod style;
use style::*;

fn main() {
    println!("Foreground colors:");
    println!(
        "  normal: {}{}{}{}{}{}{}{}",
        black(" black "),
        blue(" blue "),
        cyan(" cyan "),
        green(" green "),
        magenta(" magenta "),
        red(" red "),
        white(" white "),
        yellow(" yellow "),
    );
    println!(
        "  light:  {}{}{}{}{}{}{}{}",
        light_black(" black "),
        light_blue(" blue "),
        light_cyan(" cyan "),
        light_green(" green "),
        light_magenta(" magenta "),
        light_red(" red "),
        light_white(" white "),
        light_yellow(" yellow "),
    );
    println!("");
    println!("Background colors:");
    println!(
        "  normal: {}{}{}{}{}{}{}{}",
        bg_black(" black "),
        bg_blue(" blue "),
        bg_cyan(" cyan "),
        bg_green(" green "),
        bg_magenta(" magenta "),
        bg_red(" red "),
        bg_white(" white "),
        bg_yellow(" yellow "),
    );
    println!(
        "  light:  {}{}{}{}{}{}{}{}",
        bg_light_black(" black "),
        bg_light_blue(" blue "),
        bg_light_cyan(" cyan "),
        bg_light_green(" green "),
        bg_light_magenta(" magenta "),
        bg_light_red(" red "),
        bg_light_white(" white "),
        bg_light_yellow(" yellow "),
    );
}
