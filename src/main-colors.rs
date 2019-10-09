mod style;

use style::Color::*;
use style::Style::*;
use style::*;

fn main() {
    println!("Foreground colors:");
    println!(
        "  normal: {}{}{}{}{}{}{}{}",
        colored("  black  ", Black),
        colored("  blue   ", Blue),
        colored("  cyan   ", Cyan),
        colored("  green  ", Green),
        colored(" magenta ", Magenta),
        colored("   red   ", Red),
        colored("  white  ", White),
        colored(" yellow  ", Yellow),
    );
    println!(
        "  light:  {}{}{}{}{}{}{}{}",
        colored("  black  ", LightBlack),
        colored("  blue   ", LightBlue),
        colored("  cyan   ", LightCyan),
        colored("  green  ", LightGreen),
        colored(" magenta ", LightMagenta),
        colored("   red   ", LightRed),
        colored("  white  ", LightWhite),
        colored(" yellow  ", LightYellow),
    );
    println!("Background colors:");
    println!(
        "  normal: {}{}{}{}{}{}{}{}",
        bg_colored("  black  ", Black),
        bg_colored("  blue   ", Blue),
        bg_colored("  cyan   ", Cyan),
        bg_colored("  green  ", Green),
        bg_colored(" magenta ", Magenta),
        bg_colored("   red   ", Red),
        bg_colored("  white  ", White),
        bg_colored(" yellow  ", Yellow),
    );
    println!(
        "  light:  {}{}{}{}{}{}{}{}",
        bg_colored("  black  ", LightBlack),
        bg_colored("  blue   ", LightBlue),
        bg_colored("  cyan   ", LightCyan),
        bg_colored("  green  ", LightGreen),
        bg_colored(" magenta ", LightMagenta),
        bg_colored("   red   ", LightRed),
        bg_colored("  white  ", LightWhite),
        bg_colored(" yellow  ", LightYellow),
    );

    // for r in 0..6 {
    //     for g in 0..6 {
    //         for b in 0..6 {
    //             print!("{} ", termion::color::AnsiValue::rgb(r, g, b).bg_string(),);
    //         }
    //     }
    // }
    // println!("\n");
    // for g in 0..24 {
    //     print!("{} ", termion::color::AnsiValue::grayscale(g).bg_string());
    // }
    // println!("\n");
}
