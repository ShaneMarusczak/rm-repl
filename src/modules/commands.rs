use rusty_maths::{
    equation_analyzer::calculator::plot,
    linear_algebra::{vector_mean, vector_sum},
};

use rusty_maths::equation_analyzer::calculator::Point;

use crate::modules::{
    common::*,
    cube::cube,
    graphing::graph,
    inputs::{get_g_inputs, get_matrix_input, get_numerical_input},
    logger::Logger,
    repl::Repl,
    string_maker::make_table_string,
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, ExecutableCommand};

use super::{
    bezier_curve::{cubic_bezier, quadratic_bezier},
    inputs::read_user_input,
};

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
        //that inserts "you are a math tutor" or w/e to the start of each prompt

        //TODO: a fast forier transform (fft), takes a sound file, shows a report of all wave forms seen, with time ranges when heard
        "t" | "table" => t(l),
        "g" | "graph" => g(l, &go),
        "o" | "graph options" => gos(l, repl),
        "ag" | "animated graph" => ag(l, &go),
        "ig" | "interactive graph" => ig(l, &go),
        "la" | "linear algebra" => la(l),
        "c" | "cube" | "3d" => c(l, &go),
        "qbc" => qbc(l, &go),
        "cbc" => cbc(l, &go),
        "h" | "help" => h(l),
        _ => {
            l.eprint(&format!("invalid command {line}"));
        }
    }
}

fn h(l: &mut impl Logger) {
    l.print("Available commands:");
    l.print(":g  | :graph -> graphing mode");
    l.print(":t  | :table -> table mode");
    l.print(":o  | :graph options -> graph options mode");
    l.print(":ag | :animated graph -> animated graph mode");
    l.print(":ig | :interactive graph -> interactive graph mode");
    l.print(":la | :linear algebra -> linear algebra mode");
    l.print(":c  | :cube | :3d -> renders an animated cube to the terminal");
    l.print(":qbc -> quadratic bezier curve");
    l.print(":cbc -> cubic bezier curve");
    l.print(":q  | :quit -> exits the repl session");
}

fn cbc(l: &mut impl Logger, go: &GraphOptions) {
    l.print(&format!(
        "Lower Right Coordinates - x:{}, y:{}",
        go.width, go.height
    ));

    let p1_x = get_numerical_input("start x: ", l);
    let p1_y = get_numerical_input("start y: ", l);

    let p2_x = get_numerical_input("control1 x: ", l);
    let p2_y = get_numerical_input("control1 y: ", l);

    let p3_x = get_numerical_input("control2 x: ", l);
    let p3_y = get_numerical_input("control2 y: ", l);

    let p4_x = get_numerical_input("end x: ", l);
    let p4_y = get_numerical_input("end y: ", l);

    let p1 = Point::new(p1_x, p1_y);
    let p2 = Point::new(p2_x, p2_y);
    let p3 = Point::new(p3_x, p3_y);
    let p4 = Point::new(p4_x, p4_y);

    cubic_bezier(p1, p2, p3, p4, go, l);
}

fn qbc(l: &mut impl Logger, go: &GraphOptions) {
    l.print(&format!(
        "Lower Right Coordinates - x:{}, y:{}",
        go.width, go.height
    ));
    let p1_x = get_numerical_input("start x: ", l);
    let p1_y = get_numerical_input("start y: ", l);

    let p2_x = get_numerical_input("control x: ", l);
    let p2_y = get_numerical_input("control y: ", l);

    let p3_x = get_numerical_input("end x: ", l);
    let p3_y = get_numerical_input("end y: ", l);

    let p1 = Point::new(p1_x, p1_y);
    let p2 = Point::new(p2_x, p2_y);
    let p3 = Point::new(p3_x, p3_y);

    quadratic_bezier(p1, p2, p3, go, l);
}

fn c(l: &mut impl Logger, go: &GraphOptions) {
    cube(l, go);
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
    } else if let Err(p) = points {
        l.eprint(&p);
    }
}

fn g(l: &mut impl Logger, go: &GraphOptions) {
    let (eq, x_min, x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go);

    if let Ok(g) = g {
        l.print(&g);
    } else if let Err(g) = g {
        l.eprint(&g);
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

            let _ = stdout.execute(cursor::MoveUp(new_lines as u16));

            if let Ok(g) = graph(&eq, x_min - n as f32, x_max + n as f32, go) {
                l.print(&g);
            }
        }
    } else if let Err(g) = g {
        l.eprint(&g);
    }
}

fn ig(l: &mut impl Logger, go: &GraphOptions) {
    let mut stdout = std::io::stdout();

    let (eq, mut x_min, mut x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go);

    if let Ok(g) = g {
        l.print(&g);

        let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;
        let _ = enable_raw_mode();

        loop {
            match read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                })) => {
                    let _ = disable_raw_mode();
                    x_min += 1.0;
                    x_max += 1.0;
                    let _ = stdout.execute(cursor::MoveUp(new_lines as u16));

                    if let Ok(g) = graph(&eq, x_min, x_max, go) {
                        l.print(&g);
                    }

                    let _ = enable_raw_mode();
                }

                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                })) => {
                    let _ = disable_raw_mode();
                    x_min -= 1.0;
                    x_max -= 1.0;

                    let _ = stdout.execute(cursor::MoveUp(new_lines as u16));

                    if let Ok(g) = graph(&eq, x_min, x_max, go) {
                        l.print(&g);
                    }

                    let _ = enable_raw_mode();
                }

                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                })) => break,
                _ => continue,
            }
        }
        let _ = disable_raw_mode();
    } else if let Err(g) = g {
        l.eprint(&g);
    }
}

fn la(l: &mut impl Logger) {
    loop {
        if let Ok(op_code) = read_user_input("operation: ") {
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
}
