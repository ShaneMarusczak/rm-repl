use crate::{commands, evaluate, graphing, inputs, logger::Logger, repl, variables};

pub(crate) fn as_repl(l: &mut impl Logger) {
    use std::collections::HashMap;

    l.print("\n--rusty maths repl--\n");

    let mut repl = repl::Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
        variables: HashMap::new(),
    };

    loop {
        let line = inputs::get_text_input(">>", l);

        if line.is_empty() {
            continue;
        } else if let Some(stripped) = line.strip_prefix(':') {
            match stripped {
                "q" | "quit" => break,
                _ => commands::run_command(stripped, &mut repl, l),
            }
        } else if !repl.previous_answer_valid && line.contains("ans") {
            l.eprint("invalid use of 'ans'");
            continue;
        } else if variables::is_variable(&line) {
            variables::handle_var(&line, &mut repl, l);
        } else {
            evaluate::evaluate(&line, &mut repl, l);
        }
    }
}

pub(crate) fn as_cli_tool(args: &Vec<String>, l: &mut impl Logger) {
    match args[1].as_str() {
        "-e" | "--evaluate" => {
            if args.len() != 3 {
                l.eprint("Usage: rmr -e [expression]");
            } else {
                evaluate::simple_evaluate(&args[2], l);
            }
        }
        "-g" | "--graph" => {
            if args.len() != 5 {
                l.eprint("Usage: rmr -g [equation] [x-min] [x-max]");
            } else if let (Ok(x_min), Ok(x_max)) = (args[3].parse(), args[4].parse()) {
                if x_min >= x_max {
                    let g = graphing::graph(&args[2], x_min, x_max);
                    if let Ok(g) = g {
                        l.print(&g);
                    } else {
                        l.eprint(&g.unwrap_err());
                    }
                } else {
                    l.eprint(&format!(
                        "x min `{x_min}` must be less than x max `{x_max}`"
                    ));
                }
            } else {
                l.eprint(&format!(
                    "x-min: `{}` and x-max: `{}` must both be valid numbers",
                    args[3], args[4]
                ));
            }
        }
        _ => l.eprint("invalid use of rmr"),
    }
}
