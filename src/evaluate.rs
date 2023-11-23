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
        repl.previous_answer(v, true);
        l.print(&format!("{v:.2}"));
    } else {
        repl.previous_answer(0.0, false);
        l.eprint(&val.unwrap_err());
    }
}

pub(crate) fn simple_evaluate(line: &str, l: &mut impl Logger) {
    let val = calculate(line);
    if let Ok(v) = val {
        l.print(&format!("{v:.2}"));
    } else {
        l.eprint(&val.unwrap_err());
    }
}
