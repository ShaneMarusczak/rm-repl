use std::collections::HashMap;

mod commands;
mod inputs;
mod repl;
mod run;
mod utils;
mod variables;

fn main() {
    println!("math!");

    let mut repl = repl::Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
        variables: HashMap::new(),
    };

    loop {
        let line = utils::read_line();

        if line.is_empty() {
            continue;
        } else if let Some(stripped) = line.strip_prefix(':') {
            commands::run_command(stripped, &mut repl);
            continue;
        } else if !repl.previous_answer_valid && line.contains("ans") {
            eprintln!("invalid use of ans");
            continue;
        } else if variables::is_variable(&line) {
            variables::handle_var(&line, &mut repl);
        } else {
            run::run(&line, &mut repl);
        }
    }
}
