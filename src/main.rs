mod commands;
mod graphing;
mod inputs;
mod repl;
mod run;
mod string_maker;
mod structs;
mod variables;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    //allow graphing directly from command tool?
    //args greater than 2 is command tool, look for flags i.e. "rmr -g y=sin(x) -5 5"
    match args.len() {
        1 => as_repl(),
        2 => as_command_tool(&args[1]),
        _ => eprintln!("invalid use of {}", args[0]),
    }
}

fn as_command_tool(line: &str) {
    run::simple_run(line);
}

fn as_repl() {
    use std::collections::HashMap;

    println!("\n--rusty maths repl--\n");

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
