use crate::{commands, evaluate, graphing, inputs, repl, variables};

pub(crate) fn as_repl() {
    use std::collections::HashMap;

    println!("\n--rusty maths repl--\n");

    let mut repl = repl::Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
        variables: HashMap::new(),
    };

    loop {
        let line = inputs::get_text_input(">>");

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
            evaluate::evaluate(&line, &mut repl);
        }
    }
}

pub(crate) fn as_cli_tool(args: &Vec<String>) {
    match args[1].as_str() {
        "-e" | "--evaluate" => {
            if args.len() != 3 {
                eprintln!("Usage: rmr -e [expression]");
            } else {
                evaluate::simple_evaluate(&args[2]);
            }
        }
        "-g" | "--graph" => {
            if args.len() != 5 {
                eprintln!("Usage: rmr -g [equation] [x-min] [x-max]");
            } else if let (Ok(x_min), Ok(x_max)) = (args[3].parse(), args[4].parse()) {
                if x_min >= x_max {
                    let g = graphing::graph(&args[2], x_min, x_max);
                    if let Ok(g) = g {
                        println!("{g}");
                    } else {
                        eprintln!("{}", g.unwrap_err());
                    }
                } else {
                    eprintln!("x min `{x_min}` must be less than x max `{x_max}`");
                }
            } else {
                eprintln!(
                    "x-min: `{}` and x-max: `{}` must both be valid numbers",
                    args[3], args[4]
                );
            }
        }
        _ => eprintln!("invalid use of rmr"),
    }
}
