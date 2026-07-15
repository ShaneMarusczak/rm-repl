use rusty_maths::equation_analyzer::calculator::{calculate, calculate_with};

use crate::modules::{error_render, logger::Logger, repl::Repl};

pub(crate) fn evaluate(line: &str, repl: &mut Repl, l: &mut impl Logger) {
    match calculate_with(line, &repl.defs) {
        Ok(v) => {
            repl.set_ans(v);
            let p = repl.precision;
            let f_v = format!("{v:.p$}");
            let trailing = format!(".{}", "0".repeat(p));
            l.print(f_v.trim_end_matches(&trailing));
        }
        // Spans refer to the typed line, still echoed above — except for
        // errors from inside a user function's body, which reprint that
        // body instead. On error `ans` keeps its last good value.
        Err(e) => l.eprint(&error_render::render_repl_error(&e, &repl.defs)),
    }
}

pub(crate) fn simple_evaluate(line: &str, l: &mut impl Logger) {
    match calculate(line) {
        Ok(v) => {
            let f_v = format!("{v:.2}");
            l.print(f_v.trim_end_matches(".00"));
        }
        // One-shot CLI use: the input came from argv, not an echoed prompt
        // line, so reprint it and point at the error.
        Err(e) => l.eprint(&error_render::format_error_with_source(line, &e)),
    }
}
