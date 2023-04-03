use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
};

use crate::{
    inputs::{get_matrix_input, get_textual_input},
    repl::{PreviousAnswer, Repl},
};

use super::plot_utils::*;

pub(crate) fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "p" | "plot" => {
            let (eq, x_min, x_max) = get_p_inputs();
            let g = p(
                eq,
                x_min,
                x_max,
                repl.config.y_min,
                repl.config.y_max,
                repl.config.width,
                repl.config.height,
            );
            println!("{}", g);
        }
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
    println!("example: 5+5 \\\\10 ans + 5 \\\\15\n");
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

        match op_code.as_str() {
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

pub(crate) fn p(
    eq: String,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    width: usize,
    height: usize,
) -> String {
    let mut y_min = y_min;

    let mut y_max = y_max;

    let multiplier = (width / 4) as f32;

    let x_step = (x_max - x_min) / ((width as f32) * multiplier);

    //multithreaded
    let points = plot(&eq, x_min, x_max, x_step);

    if let Ok(points) = points {
        let mut matrix = make_matrix(height + 1, width + 1);

        let (y_min_actual, y_max_actual) = get_y_min_max(&points);

        y_min = y_min.max(y_min_actual);
        y_max = y_max.min(y_max_actual);

        //multithreaded
        for p in get_normalized_points(height, y_min, y_max, &points, multiplier)
            .iter()
            .filter(|p| p.1 .1 < y_max && p.1 .1 > y_min)
        {
            matrix[p.1 .0][p.0].0 = 1;
        }

        check_add_x_axis(y_min, y_max, height, &mut matrix);

        matrix.reverse();

        check_add_y_axis(x_min, x_max, width, &mut matrix);

        let braille_chars = get_braille(height, width, &mut matrix);

        get_graph_string(braille_chars, x_min, x_max, y_min, y_max)
    } else {
        points.unwrap_err()
    }
}
