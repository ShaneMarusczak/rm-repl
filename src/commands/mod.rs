use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
    utilities::abs_f32,
};

use crate::{
    inputs::{get_matrix_input, get_numerical_input, get_textual_input},
    repl::{PreviousAnswer, Repl},
};

pub(crate) fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "p" | "plot" => p(repl),
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

//refactor this out to its own module
//build integration like what the ti-84 does

fn p(repl: &mut Repl) {
    let eq = get_textual_input("equation: ");

    let x_min = get_numerical_input("x min: ");

    let x_max = get_numerical_input("x max: ");

    let width: usize = 120;
    let height: usize = 60;

    let mut y_min = repl.config.y_min;

    let mut y_max = repl.config.y_max;

    let multiplier = 10_f32;

    let x_range = x_max - x_min;

    let step_size = x_range / ((width as f32) * multiplier);

    let points = plot(&eq, x_min, x_max, step_size);

    let y_axis_in_view = x_min < 0_f32 && x_max > 0_f32;

    let y_axis_ratio: f32 = abs_f32(x_min) / (x_max - x_min);

    let y_axis_col = (y_axis_ratio * width as f32).round() as usize;

    let mut matrix = make_matrix(height + 1, width + 1);

    if let Ok(points) = points {
        let (y_min_actual, y_max_actual) = get_y_min_max(&points);

        if y_min_actual > y_min {
            y_min = y_min_actual;
        }

        if y_max_actual < y_max {
            y_max = y_max_actual;
        }

        let x_axis_in_view = y_min < 0_f32 && y_max > 0_f32;

        let x_axis_ratio = abs_f32(y_min) / (y_max - y_min);

        let x_axis_row = (x_axis_ratio * height as f32).round() as usize;

        let y_range = y_max - y_min;

        let y_step = y_range / height as f32;

        let y_values: Vec<f32> = (0..=height).map(|n| y_min + (y_step * n as f32)).collect();

        let new_points = points
            .iter()
            .enumerate()
            .map(|(i, point)| (i / multiplier as usize, get_y_value(&y_values, *point)))
            .collect::<Vec<_>>();

        for p in new_points
            .iter()
            .filter(|p| p.1 .1 < y_max && p.1 .1 > y_min)
        {
            matrix[p.1 .0][p.0].0 = 1;
        }

        if x_axis_in_view {
            if let Some(row) = matrix.get_mut(x_axis_row) {
                for c in row.iter_mut() {
                    c.0 = 1;
                }
            }
        }

        matrix.reverse();

        if y_axis_in_view {
            for row in &mut matrix {
                row[y_axis_col].0 = 1;
            }
        }

        let mut chars = Vec::with_capacity(height / 4);
        for _ in 0..(height / 4) {
            chars.push(Vec::with_capacity(width / 2));
        }
        for row in 0..matrix.len() {
            for col in 0..matrix[row].len() {
                let cell = matrix[row][col];
                if cell.1 {
                    continue;
                }
                let mut char = vec![];
                for dx in 0..=1 {
                    for dy in 0..=2 {
                        if row + dy < matrix.len() && col + dx < matrix[row].len() {
                            let val = matrix[row + dy][col + dx];
                            char.push(val.0);
                            matrix[row + dy][col + dx].1 = true;
                        }
                    }
                }
                for dx in 0..=1 {
                    let dy = 3;
                    if row + dy < matrix.len() && col + dx < matrix[row].len() {
                        let val = matrix[row + dy][col + dx];
                        char.push(val.0);
                        matrix[row + dy][col + dx].1 = true;
                    }
                }
                if (row / 4) < chars.len() {
                    char.reverse();

                    let binary_string = char.iter().map(|b| b.to_string()).collect::<String>();
                    let decimal_number = u8::from_str_radix(&binary_string, 2).unwrap();
                    let code_point =
                        u32::from_str_radix(&format!("28{:02x}", decimal_number), 16).unwrap();
                    let character = std::char::from_u32(code_point).unwrap();

                    chars[row / 4].push(character);
                }
            }
        }

        println!("{}", "-".repeat(63));
        for (i, row) in chars.iter().enumerate() {
            let mut s = String::new();
            s.push('|');
            for cell in row {
                s.push(*cell);
            }
            s.push('|');
            if i == 0 {
                s = s + &format!("{}", y_max);
            } else if i == chars.len() - 1 {
                s = s + &format!("{}", y_min);
            }
            println!("{}", s);
        }
        println!("{}", "-".repeat(63));
        println!("{}{}{}", x_min, " ".repeat(60), x_max);
    } else {
        eprintln!("{}", points.unwrap_err());
    }
}

fn get_y_min_max(points: &[(f32, f32)]) -> (f32, f32) {
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;

    for point in points {
        if point.1 < y_min {
            y_min = point.1;
        } else if point.1 > y_max {
            y_max = point.1;
        }
    }
    (y_min, y_max)
}

fn get_y_value(points: &[f32], point: (f32, f32)) -> (usize, f32) {
    let mut min_dif = f32::MAX;

    let mut rv = 0;
    for (i, p) in points.iter().enumerate() {
        let dif = abs_f32(point.1 - p);
        if dif < min_dif {
            min_dif = dif;
            rv = i;
        }
    }
    (rv, point.1)
}

fn make_matrix(arr_count: usize, arr_length: usize) -> Vec<Vec<(u8, bool)>> {
    (0..arr_count)
        .map(|_| (0..arr_length).map(|_| (0, false)).collect())
        .collect()
}
