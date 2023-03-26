use std::collections::HashMap;

use crate::run::simple_run;

mod commands;
mod inputs;
mod repl;
mod run;
mod variables;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => as_repl(),
        2 => as_command_tool(&args[1]),
        _ => eprintln!("invalid use of {}", args[0]),
    }
}

fn as_command_tool(line: &str) {
    simple_run(line);
}

fn as_repl() {
    println!("\n--rusty maths repl--\n");

    let mut repl = repl::Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
        variables: HashMap::new(),
        config: load_config(),
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

fn load_config() -> repl::RmrConfig {
    let file_content = std::fs::read_to_string("config.toml").unwrap();
    toml::from_str(&file_content).unwrap()
}
