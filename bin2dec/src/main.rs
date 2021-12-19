use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let intval = isize::from_str_radix(config.binval.trim(), 2)
        .expect("Check numbers. 0 or 1");

    println!("2進数: {} -> 10進数: {}", args[1], intval);
}

struct Config {
    binval: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let binval = args[1].clone();
        
        Ok(Config { binval })
    }
}