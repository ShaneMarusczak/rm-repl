use rusty_maths::equation_analyzer::calculator::calculate;

use crate::modules::{error_render, logger::Logger, repl::Repl, variables};

pub(crate) fn evaluate(line: &str, repl: &mut Repl, l: &mut impl Logger) {
    let line_internal = variables::insert_ans_vars(line, repl);

    match calculate(&line_internal) {
        Ok(v) => {
            repl.set_previous_answer(&v);
            let p = repl.precision;
            let f_v = format!("{v:.p$}");
            let trailing = format!(".{}", "0".repeat(p));
            l.print(f_v.trim_end_matches(&trailing));
        }
        Err(e) => {
            repl.invalidate_prev_answer();
            // Error spans refer to `line_internal`. When `ans`/variable
            // substitution changed the text, the echoed line no longer
            // matches, so reprint the evaluated text and point at that.
            if line_internal == line {
                l.eprint(&error_render::format_error(
                    &e,
                    Some(error_render::REPL_PROMPT_WIDTH),
                ));
            } else {
                l.eprint(&error_render::format_error_with_source(&line_internal, &e));
            }
        }
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
