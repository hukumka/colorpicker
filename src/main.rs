use reformation::Reformation;
use std::str::FromStr;
use structopt::StructOpt;
use yansi::{Color, Paint};

#[derive(Reformation, Debug)]
enum ColorRange {
    #[reformation(r"\A{}\z")]
    Fixed(u8),
    #[reformation(r"\A{}..{}\z")]
    Range(u8, u8),
}

impl FromStr for ColorRange {
    type Err = reformation::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ColorRange::parse((|s| s)(s))
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
/// Utility to show how certain term colors look
struct Attr {
    #[structopt(name = "COLOR")]
    /// List of color to be displayed
    ///
    /// Colors can be specified by color id,
    /// or by specifing range of colors:
    ///     `colorpicker 1 2 4..6 8`
    /// will display colors with id: 1, 2, 4, 5, 6, 8
    colors: Vec<ColorRange>,
}

impl ColorRange {
    fn print_all(&self, _attr: &Attr) {
        match *self {
            ColorRange::Fixed(c) => {
                print_color(c);
            }
            ColorRange::Range(a, b) => {
                if a < b {
                    for c in a..=b {
                        print_color(c);
                    }
                } else {
                    for c in b..=a {
                        print_color(c);
                    }
                }
            }
        }
    }
}

fn print_color(c: u8) {
    print!("{}={} ", c, Paint::new("__").bg(Color::Fixed(c)));
}

fn main() {
    let opt = Attr::from_args();

    for color in &opt.colors {
        color.print_all(&opt);
    }
    println!();
}
