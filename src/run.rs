use std::cmp::Ordering;

use rusty_maths::equation_analyzer::calculator::plot;

use crate::{
    commands, evaluate, graphing, inputs, logger::Logger, repl, string_maker::make_table_string,
    variables,
};

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
    match args.len().cmp(&2) {
        Ordering::Equal => evaluate::simple_evaluate(&args[1], l),

        Ordering::Greater => match args[1].as_str() {
            "-g" | "--graph" => {
                if args.len() != 5 {
                    l.eprint("Usage: rmr -g [equation] [x-min] [x-max]");
                } else if let (Ok(x_min), Ok(x_max)) = (args[3].parse(), args[4].parse()) {
                    if x_min < x_max {
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
            "-t" | "--table" => {
                if args.len() != 6 {
                    l.eprint("Usage: rmr -t [equation] [x-min] [x-max] [step_size]");
                } else if let (Ok(x_min), Ok(x_max), Ok(step_size)) =
                    (args[3].parse(), args[4].parse(), args[5].parse())
                {
                    if x_min < x_max {
                        let points = plot(&args[2], x_min, x_max, step_size);
                        if let Ok(points) = points {
                            let t = make_table_string(points);
                            l.print(&t);
                        } else {
                            l.eprint(&points.unwrap_err());
                        }
                    } else {
                        l.eprint(&format!(
                            "x min `{x_min}` must be less than x max `{x_max}`"
                        ));
                    }
                } else {
                    l.eprint(&format!(
                        "x-min: `{}`, x-max: `{}` and step_size: `{}` must all be valid numbers",
                        args[3], args[4], args[5]
                    ));
                }
            }
            _ => l.eprint("invalid use of rmr"),
        },

        Ordering::Less => l.eprint("Usage: rmr [expression]"),
    }
}
