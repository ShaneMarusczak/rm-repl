use std::process::exit;

use rusty_maths::{equation_analyzer::calculator::calculate, linear_algebra::Matrix};
use rustyline::error::ReadlineError;

pub(crate) fn get_matrix_input() -> Matrix {
    let vec_amount: usize = get_numerical_input("vector count: ");
    let mut m: Matrix = Vec::with_capacity(vec_amount);
    let entry_fn = get_text_input("entry fn: ");
    for i in 0..vec_amount {
        let vec_size: usize = get_numerical_input(&format!("vector {} size: ", i));
        m.push(Vec::with_capacity(vec_size));
        for x in 0..vec_size {
            let e_f = entry_fn.replace('x', &x.to_string());
            m[i].push(calculate(&e_f).unwrap() as f64);
        }
    }
    m
}

pub(crate) fn get_numerical_input<T>(msg: &str) -> T
where
    T: num_traits::Num,
{
    loop {
        let s = get_text_input(msg);

        if let Ok(x) = <T>::from_str_radix(&s, 10) {
            return x;
        }
        eprintln!("{s} is not a valid number");
    }
}

pub(crate) fn get_text_input(msg: &str) -> String {
    let mut rl = rustyline::Editor::<()>::new().unwrap();
    let readline = rl.readline(msg);
    match readline {
        Ok(line) => line.trim().to_owned(),
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => exit(0),
        Err(err) => {
            eprintln!("Error: {err:?}");
            exit(0);
        }
    }
}

pub(crate) fn get_g_inputs() -> (String, f32, f32) {
    let eq = get_text_input("equation: ");

    let mut x_min = get_numerical_input("x min: ");

    let mut x_max = get_numerical_input("x max: ");

    while x_min >= x_max {
        eprintln!("x min `{x_min}` must be less than x max `{x_max}`");

        x_min = get_numerical_input("x min: ");

        x_max = get_numerical_input("x max: ");
    }

    (eq, x_min, x_max)
}
