use rusty_maths::equation_analyzer::calculator::calculate;

use crate::repl;

pub(crate) fn handle_var(str: &str, repl: &mut repl::Repl) {
    let mut iter = str.chars();
    let name = iter.next().unwrap();

    let mut exp: String = iter.filter(|c| !c.eq(&'=')).collect();

    exp = insert_ans_vars(&exp, repl);

    if let Ok(v) = calculate(exp.trim()) {
        repl.variables.insert(name, v.to_string());
    } else {
        eprintln!("invalid value");
    }
}

pub(crate) fn insert_ans_vars(str: &str, repl: &repl::Repl) -> String {
    let mut str = str.to_owned();

    if repl.previous_answer_valid && str.contains("ans") {
        str = str.replace("ans", &repl.previous_answer.to_string());
    }

    if !repl.variables.is_empty() {
        for k in repl.variables.keys() {
            let from = k.to_string();
            let to = repl.variables.get(k).unwrap();
            str = str.replace(&from, to);
        }
    }
    str
}

pub(crate) fn is_variable(str: &str) -> bool {
    let str = str.to_owned();
    if !str.starts_with(|c: char| c.is_alphabetic() && c.is_uppercase()) || str.len() < 2 {
        return false;
    }
    let second_is_equal = str.chars().nth(1).unwrap().eq(&'=');
    let second_is_space_and_third_char_is_equal_sign =
        str.chars().nth(1).unwrap().eq(&' ') && str.chars().nth(2).unwrap().eq(&'=');

    second_is_equal || second_is_space_and_third_char_is_equal_sign
}
