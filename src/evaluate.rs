use rusty_maths::equation_analyzer::calculator::calculate;

use crate::{
    logger::Logger,
    repl::{PreviousAnswer, Repl},
    variables,
};

pub(crate) fn evaluate(line: &str, repl: &mut Repl, l: &mut impl Logger) {
    let line_internal = variables::insert_ans_vars(line, repl);

    let val = calculate(&line_internal);
    if let Ok(v) = val {
        repl.set_previous_answer(&v);
        let f_v = format!("{v:.2}");
        l.print(f_v.trim_end_matches(".00"));
    } else {
        repl.invalidate_prev_answer();
        l.eprint(&val.unwrap_err());
    }
}

pub(crate) fn simple_evaluate(line: &str, l: &mut impl Logger) {
    let val = calculate(line);
    if let Ok(v) = val {
        let f_v = format!("{v:.2}");
        l.print(f_v.trim_end_matches(".00"));
    } else {
        l.eprint(&val.unwrap_err());
    }
}
