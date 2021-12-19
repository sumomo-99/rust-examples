use std::fs::read_to_string;
use structopt::StructOpt;
use  rayon::prelude::*;

#[derive(StructOpt)]
#[structopt(name = "rgrep")]
struct GrepArgs {
    #[structopt(short = "n", long = "line-number", help = "Prints line number")]
    number: bool,
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "FILE")]
    path: Vec<String>,
}

fn grep(content: &str, number: &bool, pattern: &str, path: &str) {
    for (i, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            match number {
                true => println!("{} {}: {}", i + 1, path, line),
                _ => println!("{}: {}", path, line),
            }
            
        }
    }
}

fn run(state: GrepArgs) {
    state
        .path
        .par_iter()
        .for_each(|file| match read_to_string(file) {
            Ok(content) => grep(&content, &state.number, &state.pattern, file),
            Err(reason) => println!("{}", reason),
        });
}

fn main() {
    run(GrepArgs::from_args());
}