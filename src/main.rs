use std::io::{self, Write};

mod commands;
mod repl;
mod run;

fn main() {
    println!("  math!\n");

    let mut repl = repl::Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
    };

    loop {
        print!("> ");
        io::stdout().flush().unwrap_or_default();

        let line = read_line();

        if line.is_empty() {
            continue;
        } else if let Some(stripped) = line.strip_prefix(':') {
            commands::run_command(stripped, &mut repl);
            continue;
        } else if !repl.previous_answer_valid && line.contains("ans") {
            eprintln!("invalid use of ans");
            continue;
        } else {
            run::run(
                &line.replace("ans", &repl.previous_answer.to_string()),
                &mut repl,
            );
        }
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");

    let line_trim = line.trim();

    String::from(line_trim)
}
