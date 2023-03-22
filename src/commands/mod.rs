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
    const WIDTH: usize = 120;
    const HEIGHT: usize = 60;

    let eq: String = get_textual_input("equation: ");

    let x_min = get_numerical_input("x min: ");

    let x_max = get_numerical_input("x max: ");

    let step_size = (x_max - x_min) / WIDTH as f32;

    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;

    let points = plot(eq.trim(), x_min, x_max, step_size);

    let mut matrix = make_matrix(HEIGHT + 1, WIDTH + 1);

    if let Ok(points) = points {
        for point in &points {
            if point.1 < y_min {
                y_min = point.1;
            } else if point.1 > y_max {
                y_max = point.1;
            }
        }

        let y_range = y_max - y_min;

        let y_step = y_range / HEIGHT as f32;

        let mut y_values = Vec::with_capacity(60);

        for n in 0..=HEIGHT {
            let value = y_min + (y_step * n as f32);
            y_values.push(value);
        }
        let mut new_points = vec![];
        for (i, point) in points.iter().enumerate() {
            let x = i;
            let y = get_y_2(&y_values, *point);
            new_points.push((x, y));
        }

        for p in new_points {
            matrix[p.1 .0][p.0].0 = 1;
            matrix[p.1 .1][p.0].0 = 1;
        }
        matrix.reverse();

        #[derive(Debug)]
        struct BC {
            pattern: Vec<u8>,
        }
        let mut chars = Vec::with_capacity(HEIGHT / 4);
        for _ in 0..(HEIGHT / 4) {
            chars.push(Vec::with_capacity(WIDTH / 2));
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
                            let val = matrix[i + y][j + x];
                            char.pattern.push(val.0);
                            matrix[i + y][j + x].1 = true;
                        }
                    }
                }
                for x in 0..=1 {
                    let y = 3;
                    if i + y < matrix.len() && j + x < matrix[i].len() {
                        let val = matrix[i + y][j + x];
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

                    chars[i / 4].push(character);
                }
            }
        }

        println!("---------------------------------------------------------------");
        for (i, row) in chars.iter().enumerate() {
            let mut s = String::new();
            s.push('|');
            s.push(' ');
            for cell in row {
                s.push(*cell);
            }
            s.push(' ');
            s.push('|');
            if i == 0 {
                s = s + &format!("{}", y_max);
            } else if i == chars.len() - 1 {
                s = s + &format!("{}", y_min);
            }
            println!("{}", s);
        }
        println!("---------------------------------------------------------------");
        println!(
            "{}                                                             {}",
            x_min, x_max
        );
    } else {
        eprintln!("{}", points.unwrap_err());
    }
}

fn _get_y(points: &[f32], point: (f32, f32)) -> usize {
    let mut min_dif = f32::MAX;

    let mut rv = 0;
    for (i, p) in points.iter().enumerate() {
        let dif = abs_f32(point.1 - p);
        if dif < min_dif {
            min_dif = dif;
            rv = i;
        }
    }

    rv
}

fn get_y_2(points: &[f32], point: (f32, f32)) -> (usize, usize) {
    let mut min_dif = f32::MAX;
    let mut min_dif2 = f32::MAX;

    let mut rv = 0;
    let mut rv2 = 0;
    for (i, p) in points.iter().enumerate() {
        let dif = abs_f32(point.1 - p);
        if dif < min_dif {
            min_dif = dif;
            rv = i;
        }
    }

    for (i, p) in points.iter().enumerate() {
        let dif = abs_f32(point.1 - p);
        if dif < min_dif2 && dif > min_dif {
            min_dif2 = dif;
            rv2 = i;
        }
    }

    (rv, rv2)
}

fn make_matrix(arr_count: usize, arr_length: usize) -> Vec<Vec<(u8, bool)>> {
    (0..arr_count)
        .map(|_| (0..arr_length).map(|_| (0, false)).collect())
        .collect()
}
