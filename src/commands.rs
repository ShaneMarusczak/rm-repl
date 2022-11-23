use std::{
    io::{self, Write},
    process::exit,
};

use rusty_maths::equation_analyzer::calculator::plot;
use textplots::{Chart, Plot, Shape};

use crate::repl::{PreviousAnswer, Repl};

pub(crate) fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "q" => exit(0),
        "p" => p(),
        _ => {
            eprintln!("Invalid Command");
            repl.previous_answer(0.0, false);
        }
    }
}

fn p() {
    print!("Equation: ");
    io::stdout().flush().unwrap_or_default();
    let mut eq = String::new();
    io::stdin().read_line(&mut eq).expect("Failed to read line");

    print!("x min: ");
    io::stdout().flush().unwrap_or_default();
    let mut x_mi = String::new();
    io::stdin()
        .read_line(&mut x_mi)
        .expect("Failed to read line");

    print!("x max: ");
    io::stdout().flush().unwrap_or_default();
    let mut x_mx = String::new();
    io::stdin()
        .read_line(&mut x_mx)
        .expect("Failed to read line");

    print!("step size: ");
    io::stdout().flush().unwrap_or_default();
    let mut step_sz = String::new();
    io::stdin()
        .read_line(&mut step_sz)
        .expect("Failed to read line");

    let x_min = x_mi.trim().parse::<f32>().unwrap();
    let x_max = x_mx.trim().parse::<f32>().unwrap();
    let step_size = step_sz.trim().parse::<f32>().unwrap();

    let points = plot(eq.trim(), x_min, x_max, step_size);

    if let Ok(points) = points {
        Chart::new(120, 60, x_min, x_max)
            .lineplot(&Shape::Lines(&points))
            .display();
    } else {
        eprintln!("{}", points.unwrap_err());
    }
}
