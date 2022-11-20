use rusty_maths::equation_analyzer::calculator::calculate;
use std::io::{self, Write};
use std::process::exit;

mod repl;
use repl::Repl;

fn main() {
    println!("Math!\n");

    let mut repl = Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
    };

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

        let ans_requested = line_trim.contains("ans");

        if repl.previous_answer_valid && ans_requested {
            let new_line = &line_trim.replace("ans", &repl.previous_answer.to_string());
            run(new_line, &mut repl);
        } else if !repl.previous_answer_valid && ans_requested {
            eprintln!("Invalid use of ans");
            continue;
        } else {
            run(line_trim, &mut repl);
        }
    }
}

fn run(line: &str, repl: &mut Repl) {
    let val = calculate(line);
    if let Ok(v) = val {
        repl.previous_answer = v;
        repl.previous_answer_valid = true;
        println!("{}", v);
    } else {
        repl.previous_answer = 0.0;
        repl.previous_answer_valid = false;
        eprintln!("{}", val.unwrap_err());
    }
}
