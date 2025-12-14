use rusty_maths::equation_analyzer::calculator::calculate;

use crate::modules::{logger::Logger, repl};

pub(crate) fn handle_var(str: &str, repl: &mut repl::Repl, l: &mut impl Logger) {
    let mut iter = str.chars();
    if let Some(name) = iter.next() {
        let mut exp: String = iter.filter(|c| !c.eq(&'=')).collect();

        exp = insert_ans_vars(&exp, repl);

        if let Ok(v) = calculate(exp.trim()) {
            repl.variables.insert(name, v.to_string());
        } else {
            l.eprint("Invalid variable value");
        }
    }
}

pub(crate) fn insert_ans_vars(s: &str, repl: &repl::Repl) -> String {
    let mut s = s.to_owned();

    if repl.previous_answer_valid && s.contains("ans") {
        s = s.replace("ans", &repl.previous_answer.to_string());
    }

    // Replace single-char variables only when they're not part of a word
    for (from, to) in &repl.variables {
        let mut result = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();

        for (i, &c) in chars.iter().enumerate() {
            if c == *from {
                let prev_is_alphanum = i > 0 && chars[i - 1].is_alphanumeric();
                let next_is_alphanum = i + 1 < chars.len() && chars[i + 1].is_alphanumeric();

                // Only replace if not surrounded by alphanumeric chars
                if !prev_is_alphanum && !next_is_alphanum {
                    result.push_str(to);
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
        s = result;
    }

    s
}

// pub(crate) fn is_variable_new(str: &str) -> bool {
//     if let Some(stripped) = str.strip_prefix("let ") {
//         if !stripped.starts_with('=') && stripped.contains(" = ") && !stripped.ends_with('=') {
//             let equal_sign_count = stripped.matches('=').count();
//             return equal_sign_count == 1;
//         }
//     }
//     false
// }

pub(crate) fn is_variable(str: &str) -> bool {
    if str.len() < 2 {
        return false;
    }

    let first_char = match str.chars().next() {
        Some(c) => c,
        None => return false,
    };

    if !first_char.is_alphabetic() || !first_char.is_uppercase() {
        return false;
    }

    match str.chars().nth(1) {
        Some('=') => true,
        Some(' ') => str.chars().nth(2) == Some('='),
        _ => false,
    }
}
