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
        "p" | "plot" => p(),
        "la" | "linear algebra" => la(),
        "h" | "help" => h(),
        _ => {
            eprintln!("invalid command");
            repl.previous_answer(0.0, false);
        }
    }
}

fn h() {
    println!("\nrusty maths repl help\n");
    println!("this repl will evaluate arbitrary mathematical expressions");
    println!("example: (3 * sin(90)) / sqrt(3 * 4^3)\n");
    println!("each answer returned is stored in a reserved keyword 'ans' for use in the next line");
    println!("exapmle: 5+5 \\\\10 ans + 5 \\\\15\n");
    println!("each repl session can hold variables");
    println!("variables are denoted by starting a line with a captial letter, an equal sign, then an expression that evaulates to a single value");
    println!("examples: A=1 B=3*sin(90), C=A+B\n");
    println!("commands can be used to enter into alternate modes");
    println!("commands begin with a ':' followed by the first letter of the command or the full command name");
    println!("example: :p or :plot\n");
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
