use rand::Rng;
use std::env;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

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
    let mut words: Vec<String> = env::args().collect();
    words.remove(0);

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
