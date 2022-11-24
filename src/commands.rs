use std::process::exit;

use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
};
use textplots::{Chart, Plot, Shape};

use crate::{
    inputs::{get_matrix_input, get_numberical_input, get_textual_input},
    repl::{PreviousAnswer, Repl},
};

pub(crate) fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "q" | "quit" => exit(0),
        "p" | "plot" => p(),
        "la" | "linear algebra" => la(),
        _ => {
            eprintln!("invalid command");
            repl.previous_answer(0.0, false);
        }
    }
}

fn la() {
    loop {
        let op_code = get_textual_input("operation: ");

        match op_code.trim() {
            "vs" | "vector sum" => {
                let m = get_matrix_input();
                let sum = vector_sum(&m);
                println!("{:#?}", sum);
            }
            "vm" | "vector mean" => {
                let m = get_matrix_input();
                let sum = vector_mean(&m);
                println!("{:#?}", sum);
            }
            "b" | "back" => break,
            _ => eprintln!("invalid operation"),
        }
    }
}

fn p() {
    let eq: String = get_textual_input("equation: ");

    let x_min: f32 = get_numberical_input("x min: ");

    let x_max: f32 = get_numberical_input("x max: ");

    let step_size: f32 = get_numberical_input("step size: ");

    let points = plot(eq.trim(), x_min, x_max, step_size);

    if let Ok(points) = points {
        Chart::new(120, 60, x_min, x_max)
            .lineplot(&Shape::Lines(&points))
            .display();
    } else {
        eprintln!("{}", points.unwrap_err());
    }
}
