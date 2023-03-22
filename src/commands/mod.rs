use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
    utilities::abs_f32,
};
use textplots::{Chart, Plot, Shape};

use crate::{
    inputs::{get_matrix_input, get_numerical_input, get_textual_input},
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
    let width = 120;
    let height = 60;

    let eq: String = get_textual_input("equation: ");

    let x_min = get_numerical_input("x min: ");

    let x_max = get_numerical_input("x max: ");

    let step_size = (x_max - x_min) / width as f32;

    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;

    let points = plot(eq.trim(), x_min, x_max, step_size);

    let mut matrix = make_matrix(height + 1, width + 1);

    if let Ok(points) = points {
        Chart::new(120, 60, x_min, x_max)
            .lineplot(&Shape::Lines(&points))
            .display();

        for point in &points {
            if point.1 < y_min {
                y_min = point.1;
            } else if point.1 > y_max {
                y_max = point.1;
            }
        }

        let y_range = y_max - y_min;

        let y_step = y_range / height as f32;

        let mut y_values = Vec::with_capacity(60);

        for n in 0..=height {
            let value = y_min + (y_step * n as f32);
            y_values.push(value);
        }
        let mut new_points = vec![];
        for (i, point) in points.iter().enumerate() {
            let x = i;
            let y = get_y(&y_values, *point);
            new_points.push((x, y));

            //SHOULD I ALSO GET SECOND CLOSEST?
        }

        for p in new_points {
            // println!("{:?}", p);

            matrix[p.1][p.0].0 = 1;
        }
        matrix.reverse();

        //now i need to get braille from this matrix, the 1s are the dots to show
        #[derive(Debug)]
        struct BC {
            pattern: Vec<u8>,
        }
        let mut chars = Vec::with_capacity(height / 4);
        for _ in 0..(height / 4) {
            chars.push(Vec::with_capacity(width / 2));
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let cell = matrix[i][j];
                if cell.1 {
                    continue;
                }
                let mut char = BC { pattern: vec![] };
                for x in 0..=1 {
                    for y in 0..=2 {
                        if i + y < matrix.len() && j + x < matrix[i].len() {
                            let val = matrix[i + y][j + x].clone();
                            char.pattern.push(val.0);
                            matrix[i + y][j + x].1 = true;
                        }
                    }
                }
                for x in 0..=1 {
                    let y = 3;
                    if i + y < matrix.len() && j + x < matrix[i].len() {
                        let val = matrix[i + y][j + x].clone();
                        char.pattern.push(val.0);
                        matrix[i + y][j + x].1 = true;
                    }
                }
                if (i / 4) < chars.len() {
                    char.pattern.reverse();

                    let binary_string = char
                        .pattern
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<String>();
                    let decimal_number = u8::from_str_radix(&binary_string, 2).unwrap();
                    let hex_string = format!("{:02x}", decimal_number);

                    let braille_char = String::from("28") + &hex_string;

                    let code_point = u32::from_str_radix(&braille_char, 16).unwrap();
                    let character = std::char::from_u32(code_point).unwrap();
                    // println!("{}", character);

                    // println!("{}", braille_char);

                    chars[i / 4].push(character);
                }
            }
        }
        // for n in 0..chars.len() {
        //     println!("{:?}", chars[n]);
        // }
        // for row in matrix {
        //     let mut s = String::new();
        //     for cell in row {
        //         if cell.0 == 0 {
        //             s.push(' ');
        //         } else {
        //             s.push('.');
        //         }
        //     }
        //     println!("{}", s);
        // }
        for row in chars {
            let mut s = String::new();
            for cell in row {
                s.push(cell);
            }
            println!("{}", s);
        }
    } else {
        eprintln!("{}", points.unwrap_err());
    }
}

fn get_y(points: &Vec<f32>, point: (f32, f32)) -> usize {
    let mut min_dif = f32::MAX;
    let mut rv = 0;
    for (i, p) in points.iter().enumerate() {
        let dif = abs_f32(point.1 - p);
        if dif < min_dif {
            min_dif = dif;
            rv = i;
        }
    }

    return rv;
}

fn make_matrix(arr_count: usize, arr_length: usize) -> Vec<Vec<(u8, bool)>> {
    let mut outer = Vec::with_capacity(arr_count);
    for _ in 0..arr_count {
        let mut inner = Vec::with_capacity(arr_length);
        for _ in 0..arr_length {
            inner.push((0, false));
        }
        outer.push(inner);
    }
    outer
}
