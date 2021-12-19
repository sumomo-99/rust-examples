use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rcat")]

struct Opt {
    #[structopt(short, long, help = "Prints line number")]
    number: bool,
    path: Vec<String>,
}

fn cat(content: &str, number: &bool) {
    for (i, line) in content.lines().enumerate() {
        match number {
            true => println!("{} {}",  i + 1, line),
            _ => println!("{}", line),
        }
    }
}

fn run(state: Opt) {
    for path in state.path {
        match read_to_string(path) {
            Ok(content) => cat(&content, &state.number),
            Err(reason) => println!("{}", reason), 
        }
    }
}

fn main() {
    run(Opt::from_args());
}