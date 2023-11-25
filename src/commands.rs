use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
};

use crate::logger::Logger;

use crate::{
    graphing::graph,
    inputs::{get_g_inputs, get_matrix_input, get_numerical_input, get_text_input},
    repl::{PreviousAnswer, Repl},
    string_maker::make_table_string,
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, ExecutableCommand};

pub(crate) fn run_command(line: &str, repl: &mut Repl, l: &mut impl Logger) {
    match line {
        "t" | "table" => t(l),
        "g" | "graph" => g(l),
        "ag" | "animated graph" => ag(l),
        "ig" | "interactive graph" => ig(l),
        "la" | "linear algebra" => la(l),
        _ => {
            l.eprint(&format!("invalid command {line}"));
            repl.invalidate_prev_answer();
        }
    }
}

fn t(l: &mut impl Logger) {
    let (eq, x_min, x_max) = get_g_inputs(l);
    let step_size = get_numerical_input("step size: ", l);

    let points = plot(&eq, x_min, x_max, step_size);

    if let Ok(points) = points {
        l.print(&make_table_string(points));
    } else {
        l.eprint(&points.unwrap_err());
    }
}

fn g(l: &mut impl Logger) {
    let (eq, x_min, x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max);

    if let Ok(g) = g {
        l.print(&g);
    } else {
        l.eprint(&g.unwrap_err());
    }
}

fn ag(l: &mut impl Logger) {
    let mut stdout = std::io::stdout();

    let (eq, x_min, x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max);

    if let Ok(g) = g {
        l.print(&g);
        let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;

        for n in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(90));

            stdout
                .execute(cursor::MoveUp(new_lines.try_into().unwrap()))
                .unwrap();
            let g = graph(&eq, x_min - n as f32, x_max + n as f32).unwrap();

            l.print(&g);
        }
    } else {
        l.eprint(&g.unwrap_err());
    }
}

fn ig(l: &mut impl Logger) {
    let mut stdout = std::io::stdout();

    let (eq, mut x_min, mut x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max);

    if let Ok(g) = g {
        l.print(&g);

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

                    l.print(&g);
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

                    l.print(&g);
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
        l.eprint(&g.unwrap_err());
    }
}

fn la(l: &mut impl Logger) {
    loop {
        let op_code = get_text_input("operation: ", l);

        match op_code.as_str() {
            "vs" | "vector sum" => {
                let m = get_matrix_input(l);
                let sum = vector_sum(&m);
                l.print(&format!("{sum:#?}"));
            }
            "vm" | "vector mean" => {
                let m = get_matrix_input(l);
                let sum = vector_mean(&m);
                l.print(&format!("{sum:#?}"));
            }
            "b" | "back" => break,
            _ => l.eprint("invalid operation"),
        }
    }
}
