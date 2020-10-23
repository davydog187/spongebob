use clap::Clap;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::Rng;

/// SPongEbOB IS BuiLT WiTH lOve foR aLl YOuR SarCAsTIc NEEds
#[derive(Clap)]
#[clap(version = "1.0", author = "Dave Lucia <davelucianyc@gmail.com>")]
struct Opts {
    /// SAy WhAT nEEDs tO BE saId
    input: Vec<String>,
}

fn spongebob(word: &str) -> String {
    let mut rng = rand::thread_rng();

    String::from(word)
        .chars()
        .map(|c| match rng.gen_range(0, 2) {
            0 => c.to_uppercase().to_string(),
            1 => c.to_lowercase().to_string(),
            _ => panic!("bad random number"),
        })
        .collect()
}

fn clippy(output: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(output.to_owned()).unwrap();
}

fn main() {
    let opts = Opts::parse();

    let words: Vec<String> = opts.input;

    let output = match words.as_slice() {
        [] => spongebob("you're doing it wrong"),
        words => words
            .iter()
            .map(|word| spongebob(word))
            .fold(String::new(), |s, word| s + &word + " "),
    };
    clippy(&output);
    println!("{}", output);
}
