use std::collections::HashMap;

mod commands;
mod inputs;
mod repl;
mod run;
mod variables;

fn main() {
    println!("\n--rusty math repl--\n");

    let mut repl = repl::Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
        variables: HashMap::new(),
    };

    loop {
        let line = inputs::get_textual_input(">>");

        if line.is_empty() {
            continue;
        } else if let Some(stripped) = line.strip_prefix(':') {
            match stripped {
                "q" | "quit" => break,
                _ => commands::run_command(stripped, &mut repl),
            }
        } else if !repl.previous_answer_valid && line.contains("ans") {
            eprintln!("invalid use of 'ans'");
            continue;
        } else if variables::is_variable(&line) {
            variables::handle_var(&line, &mut repl);
        } else {
            run::run(&line, &mut repl);
        }
    }
}
