use rusty_maths::equation_analyzer::calculator::calculate;
use std::io::{self, Write};

fn main() {
    println!("Math!");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.len() == 0 {
            // break;
        }
        run(&line);
    }
}

fn run(line: &str) {
    let val = calculate(line);
    if val.is_ok() {
        println!("{}", val.unwrap());
    } else {
        println!("{}", val.unwrap_err());
    }
}
