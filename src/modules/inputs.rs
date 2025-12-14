use rusty_maths::{equation_analyzer::calculator::calculate, linear_algebra::Matrix};
use std::error::Error;

use crate::modules::logger::Logger;

use linefeed::{Interface, ReadResult};

pub(crate) fn get_matrix_input(l: &mut impl Logger) -> Matrix {
    let vec_amount: usize = get_numerical_input("vector count: ", l);
    let mut m: Matrix = Vec::with_capacity(vec_amount);
    if let Ok(entry_fn) = read_user_input("entry fn: ") {
        for i in 0..vec_amount {
            let vec_size: usize = get_numerical_input(&format!("vector {} size: ", i), l);
            m.push(Vec::with_capacity(vec_size));
            for x in 0..vec_size {
                let e_f = entry_fn.replace('x', &x.to_string());
                if let Ok(v) = calculate(&e_f) {
                    m[i].push(v as f64);
                }
            }
        }
    }
    m
}

pub(crate) fn get_numerical_input<T>(msg: &str, l: &mut impl Logger) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    loop {
        match read_user_input(msg) {
            Ok(s) => match s.parse::<T>() {
                Ok(x) => return x,
                Err(_) => l.eprint(&format!("'{s}' is not a valid number")),
            },
            Err(e) => l.eprint(&format!("Failed to read input: {e}")),
        }
    }
}

pub fn read_user_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    let interface = Interface::new("rmr-input")?;
    interface.set_prompt(prompt)?;
    loop {
        match interface.read_line()? {
            ReadResult::Input(line) => return Ok(line.trim().to_string()),
            ReadResult::Eof => return Err("End of input".into()),
            ReadResult::Signal(_) => continue,
        }
    }
}

pub(crate) fn get_g_inputs(l: &mut impl Logger) -> (String, f32, f32) {
    loop {
        match read_user_input("equation: ") {
            Ok(eq) => {
                let mut x_min = get_numerical_input("x min: ", l);
                let mut x_max = get_numerical_input("x max: ", l);

                while x_min >= x_max {
                    l.eprint(&format!(
                        "x min `{x_min}` must be less than x max `{x_max}`"
                    ));

                    x_min = get_numerical_input("x min: ", l);
                    x_max = get_numerical_input("x max: ", l);
                }

                return (eq, x_min, x_max);
            }
            Err(e) => {
                l.eprint(&format!("Failed to read equation: {e}"));
                continue;
            }
        }
    }
}
