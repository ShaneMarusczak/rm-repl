use rusty_maths::equation_analyzer::calculator::calculate;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    println!("Math!");
    loop {
        print!("> ");
        io::stdout().flush().unwrap_or_default();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line_trim = line.trim();

        if line_trim.is_empty() {
            continue;
        }

        if line_trim.eq("q") {
            exit(0);
        }

        run(line_trim);
    }
}

fn run(line: &str) {
    let val = calculate(line);
    if val.is_ok() {
        println!("{}", val.unwrap());
    } else {
        eprintln!("{}", val.unwrap_err());
    }
}
