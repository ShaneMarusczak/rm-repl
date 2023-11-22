use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
};

use crate::{
    graphing::graph,
    inputs::{get_matrix_input, get_numerical_input, get_textual_input},
    repl::{PreviousAnswer, Repl},
    string_maker::make_table_string,
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, ExecutableCommand};
use std::io::Write;

pub(crate) fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "t" | "table" => t(),
        "g" | "graph" => g(),
        "ag" | "animated graph" => ag(),
        "ig" | "interactive graph" => ig(),
        "la" | "linear algebra" => la(),
        _ => {
            eprintln!("invalid command {line}");
            repl.previous_answer(0.0, false);
        }
    }
}

fn t() {
    let (eq, x_min, x_max) = get_g_inputs();
    let step_size = get_numerical_input("step size: ");

    let points = plot(&eq, x_min, x_max, step_size);

    if let Ok(points) = points {
        println!("{}", make_table_string(points));
    } else {
        eprintln!("{}", points.unwrap_err());
    }
}

fn g() {
    let (eq, x_min, x_max) = get_g_inputs();
    let g = graph(&eq, x_min, x_max);
    if let Ok(g) = g {
        println!("{g}");
    } else {
        eprintln!("{}", g.unwrap_err());
    }
}

fn ag() {
    let mut stdout = std::io::stdout();
    let (eq, x_min, x_max) = get_g_inputs();
    let g = graph(&eq, x_min, x_max);

    if let Ok(g) = g {
        writeln!(stdout, "{g}").unwrap();
        let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;

        for n in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(90));

            stdout
                .execute(cursor::MoveUp(new_lines.try_into().unwrap()))
                .unwrap();
            let g = graph(&eq, x_min - n as f32, x_max + n as f32).unwrap();

            writeln!(stdout, "{g}").unwrap();
        }
    } else {
        eprintln!("{}", g.unwrap_err());
    }
}

fn ig() {
    let mut stdout = std::io::stdout();
    let (eq, mut x_min, mut x_max) = get_g_inputs();
    let g = graph(&eq, x_min, x_max);

    if let Ok(g) = g {
        writeln!(stdout, "{g}").unwrap();

        let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;
        enable_raw_mode().unwrap();

        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => {
                    disable_raw_mode().unwrap();
                    x_min += 1.0;
                    x_max += 1.0;
                    stdout
                        .execute(cursor::MoveUp(new_lines.try_into().unwrap()))
                        .unwrap();
                    let g = graph(&eq, x_min, x_max).unwrap();

                    writeln!(stdout, "{g}").unwrap();
                    enable_raw_mode().unwrap();
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => {
                    disable_raw_mode().unwrap();
                    x_min -= 1.0;
                    x_max -= 1.0;
                    stdout
                        .execute(cursor::MoveUp(new_lines.try_into().unwrap()))
                        .unwrap();
                    let g = graph(&eq, x_min, x_max).unwrap();

                    writeln!(stdout, "{g}").unwrap();
                    enable_raw_mode().unwrap();
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => break,
                _ => continue,
            }
        }
        disable_raw_mode().unwrap();
    } else {
        eprintln!("{}", g.unwrap_err());
    }
}

fn la() {
    loop {
        let op_code = get_textual_input("operation: ");

        match op_code.as_str() {
            "vs" | "vector sum" => {
                let m = get_matrix_input();
                let sum = vector_sum(&m);
                println!("{sum:#?}");
            }
            "vm" | "vector mean" => {
                let m = get_matrix_input();
                let sum = vector_mean(&m);
                println!("{sum:#?}");
            }
            "b" | "back" => break,
            _ => eprintln!("invalid operation"),
        }
    }
}

fn get_g_inputs() -> (String, f32, f32) {
    let eq = get_textual_input("equation: ");

    let x_min = get_numerical_input("x min: ");

    let x_max = get_numerical_input("x max: ");

    (eq, x_min, x_max)
}
