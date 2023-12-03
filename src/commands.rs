use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
};

use crate::{logger::Logger, structs::GraphOptions};

use crate::{
    graphing::graph,
    inputs::{get_g_inputs, get_matrix_input, get_numerical_input, get_text_input},
    repl::Repl,
    string_maker::make_table_string,
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, ExecutableCommand};

pub(crate) fn run_command(line: &str, l: &mut impl Logger, repl: &mut Repl) {
    let go = GraphOptions {
        y_min: -7.,
        y_max: 7.,
        width: repl.width,
        height: repl.height,
    };
    match line {
        //TODO: scrollable graph (sg), like iteractive graph but you move a point along the graph instead of moving the graph
        //left right moves the point, up down switches graphs (if multiple)

        //TODO: add math tutor (mt) option that starts a chat session with chat gpt
        "t" | "table" => t(l),
        "g" | "graph" => g(l, &go),
        "go" | "graph options" => gos(l, repl),
        "ag" | "animated graph" => ag(l, &go),
        "ig" | "interactive graph" => ig(l, &go),
        "la" | "linear algebra" => la(l),
        _ => {
            l.eprint(&format!("invalid command {line}"));
        }
    }
}

fn gos(l: &mut impl Logger, repl: &mut Repl) {
    repl.update_dimensions(get_numerical_input("width: ", l));
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

fn g(l: &mut impl Logger, go: &GraphOptions) {
    let (eq, x_min, x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go);

    if let Ok(g) = g {
        l.print(&g);
    } else {
        l.eprint(&g.unwrap_err());
    }
}

fn ag(l: &mut impl Logger, go: &GraphOptions) {
    let mut stdout = std::io::stdout();

    let (eq, x_min, x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go);

    if let Ok(g) = g {
        l.print(&g);
        let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;

        for n in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(90));

            stdout
                .execute(cursor::MoveUp(new_lines.try_into().unwrap()))
                .unwrap();
            let g = graph(&eq, x_min - n as f32, x_max + n as f32, go).unwrap();

            l.print(&g);
        }
    } else {
        l.eprint(&g.unwrap_err());
    }
}

fn ig(l: &mut impl Logger, go: &GraphOptions) {
    let mut stdout = std::io::stdout();

    let (eq, mut x_min, mut x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go);

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
                    let g = graph(&eq, x_min, x_max, go).unwrap();

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
                    let g = graph(&eq, x_min, x_max, go).unwrap();

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
