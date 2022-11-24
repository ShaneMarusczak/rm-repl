use std::io::{self, Write};

use rusty_maths::{equation_analyzer::calculator::calculate, linear_algebra::Matrix};

pub(crate) fn get_matrix_input() -> Matrix {
    let vec_amount: usize = get_numberical_input("vector count: ");
    let mut m: Matrix = Vec::with_capacity(vec_amount);
    let entry_fn = get_textual_input("entry fn: ");
    for i in 0..vec_amount {
        let vec_size: usize = get_numberical_input(&format!("vector {} size: ", i));
        m.push(Vec::with_capacity(vec_size));
        for x in 0..vec_size {
            let e_f = entry_fn.replace('x', &x.to_string());
            m[i].push(calculate(&e_f).unwrap() as f64);
        }
    }
    m
}

pub(crate) fn get_numberical_input<T>(msg: &str) -> T
where
    T: std::str::FromStr,
{
    loop {
        print!("{}", msg);
        io::stdout().flush().unwrap_or_default();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("failed to read line");

        if let Ok(x) = s.trim().parse::<T>() {
            return x;
        } else {
            continue;
        };
    }
}

pub(crate) fn get_textual_input(msg: &str) -> String {
    let mut text = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap_or_default();
    io::stdin()
        .read_line(&mut text)
        .expect("failed to read line");
    text.trim().to_string()
}
