use clap::Clap;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::rngs::ThreadRng;
use std::str::FromStr;

/// SPongEbOB IS BuiLT WiTH lOve foR aLl YOuR SarCAsTIc NEEds
#[derive(Clap)]
#[clap(version = "1.0", author = "Dave Lucia <davelucianyc@gmail.com>")]
struct Opts {
    /// SAy WhAT nEEDs tO BE saId
    input: Vec<String>,
    /// quiet, loud or normal
    #[clap(short, long)]
    style: Option<Style>,
}

#[derive(Copy, Clone)]
enum Style {
    Quiet,
    Loud,
    Normal,
}

impl FromStr for Style {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quiet" => Ok(Style::Quiet),
            "loud" => Ok(Style::Loud),
            "normal" => Ok(Style::Normal),
            _ => Err(String::from(
                "Valid styles are quiet, loud and normal (default)",
            )),
        }
    }
}

fn silly(c: char, style: Style, mut rng: &mut ThreadRng) -> String {
    let value = match style {
        Style::Quiet => WeightedIndex::new(&[1, 3]).unwrap().sample(&mut rng),
        Style::Loud => WeightedIndex::new(&[3, 1]).unwrap().sample(&mut rng),
        Style::Normal => Uniform::from(0..2).sample(&mut rng),
    };

    match value {
        0 => c.to_uppercase().to_string(),
        1 => c.to_lowercase().to_string(),
        _ => panic!("bad random number"),
    }
}

fn spongebob(word: &str, style: Style) -> String {
    let mut rng = rand::thread_rng();

    String::from(word)
        .chars()
        .map(|c| silly(c, style, &mut rng))
        .collect()
}

fn clippy(output: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(output.to_owned()).unwrap();
}

fn main() {
    let opts = Opts::parse();

    let words: Vec<String> = opts.input;
    let style: Style = opts.style.unwrap_or(Style::Normal);

    let output = match words.as_slice() {
        [] => spongebob("you're doing it wrong", Style::Loud),
        words => words
            .iter()
            .map(|word| spongebob(word, style))
            .fold(String::new(), |s, word| s + &word + " "),
    };
    clippy(&output);
    println!("{}", output);
}
