use rusty_maths::equation_analyzer::calculator::calculate;

use crate::modules::{logger::Logger, repl};

pub(crate) fn handle_var(str: &str, repl: &mut repl::Repl, l: &mut impl Logger) {
    let mut iter = str.chars();
    let name = iter.next().unwrap();

    let mut exp: String = iter.filter(|c| !c.eq(&'=')).collect();

    exp = insert_ans_vars(&exp, repl);

    if let Ok(v) = calculate(exp.trim()) {
        repl.variables.insert(name, v.to_string());
    } else {
        l.eprint("invalid variable value");
    }
}

pub(crate) fn insert_ans_vars(s: &str, repl: &repl::Repl) -> String {
    let mut s = s.to_owned();

    if repl.previous_answer_valid && s.contains("ans") {
        s = s.replace("ans", &repl.previous_answer.to_string());
    }

    for (from, to) in &repl.variables {
        s = s.replace(*from, to);
    }

    s
}

pub(crate) fn is_variable_new(str: &str) -> bool {
    if let Some(stripped) = str.strip_prefix("let ") {
        if !stripped.starts_with('=') && stripped.contains(" = ") && !stripped.ends_with('=') {
            let equal_sign_count = stripped.matches('=').count();
            return equal_sign_count == 1;
        }
    }
    false
}

pub(crate) fn is_variable(str: &str) -> bool {
    match (
        str.len() >= 2,
        str.starts_with(char::is_alphabetic) && str.starts_with(char::is_uppercase),
    ) {
        (true, true) => match str.chars().nth(1) {
            Some('=') => true,
            Some(' ') => str.chars().nth(2) == Some('='),
            _ => false,
        },
        _ => false,
    }
}
