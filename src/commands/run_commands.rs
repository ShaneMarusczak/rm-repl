use rusty_maths::linear_algebra::{vector_mean, vector_sum};

use crate::{
    inputs::{get_matrix_input, get_textual_input},
    repl::{PreviousAnswer, Repl},
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, ExecutableCommand};
use std::io::Write;

use super::plot_utils::*;

pub(crate) fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "g" | "graph" => g(),
        "ag" | "animated graph" => ag(),
        "ig" | "interactive graph" => ig(),
        "la" | "linear algebra" => la(),
        _ => {
            eprintln!("invalid command");
            repl.previous_answer(0.0, false);
        }
    }
}

fn g() {
    let (eq, x_min, x_max) = get_p_inputs();
    let g = w(&eq, x_min, x_max, -7_f32, 7_f32, 240, 120);
    println!("{}", g);
}

fn ag() {
    let mut stdout = std::io::stdout();
    let (eq, x_min, x_max) = get_p_inputs();
    let g = w_auto(&eq, x_min, x_max, 240, 120);
    writeln!(stdout, "{}", g).unwrap();
    let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;

    for n in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(90));

        stdout
            .execute(cursor::MoveUp(new_lines.try_into().unwrap()))
            .unwrap();
        let g = w_auto(&eq, x_min - n as f32, x_max + n as f32, 240, 120);
        writeln!(stdout, "{}", g).unwrap();
    }
}

fn ig() {
    let mut stdout = std::io::stdout();
    let (eq, mut x_min, mut x_max) = get_p_inputs();
    let g = w_auto(&eq, x_min, x_max, 240, 120);
    writeln!(stdout, "{}", g).unwrap();

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
                let g = w_auto(&eq, x_min, x_max, 240, 120);
                writeln!(stdout, "{}", g).unwrap();
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
                let g = w_auto(&eq, x_min, x_max, 240, 120);
                writeln!(stdout, "{}", g).unwrap();
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
