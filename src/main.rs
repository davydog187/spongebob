use cli_clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use rand::Rng;
use std::env;

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

fn get_help() {
    println!(
        "\n{}\n\n{}\n{}\n",
        "hELp? DiD yOU RTFM?".red().reversed(),
        "Spongebob expects string arguments.",
        "`$ ./spongebob something i want to say sarcastically`".cyan()
    );
}

fn main() {
    let mut words: Vec<String> = env::args().collect();
    words.remove(0);

    if words[0] == "--help" {
        get_help();
    } else {
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
}
