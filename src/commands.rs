use std::{
    io::{self, Write},
    process::exit,
};

use rusty_maths::equation_analyzer::calculator::plot;
use textplots::{Chart, Plot, Shape};

use crate::repl::{PreviousAnswer, Repl};

pub(crate) fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "q" => exit(0),
        "p" => p(),
        _ => {
            eprintln!("invalid command");
            repl.previous_answer(0.0, false);
        }
    }
}

fn p() {
    print!("equation: ");
    io::stdout().flush().unwrap_or_default();
    let mut eq = String::new();
    io::stdin().read_line(&mut eq).expect("failed to read line");

    let x_min: f32;
    loop {
        print!("x min: ");
        io::stdout().flush().unwrap_or_default();
        let mut x_mi = String::new();
        io::stdin()
            .read_line(&mut x_mi)
            .expect("failed to read line");

        if let Ok(x) = x_mi.trim().parse::<f32>() {
            x_min = x;
            break;
        } else {
            eprintln!("unable to parse x min");
        }
    }

    let x_max: f32;

    loop {
        print!("x max: ");
        io::stdout().flush().unwrap_or_default();
        let mut x_mx = String::new();
        io::stdin()
            .read_line(&mut x_mx)
            .expect("failed to read line");

        if let Ok(x) = x_mx.trim().parse::<f32>() {
            if x <= x_min {
                eprintln!("x max cannot be equal to or less than x min");
            } else {
                x_max = x;
                break;
            }
        } else {
            eprintln!("unable to parse x max");
        }
    }

    let step_size: f32;
    loop {
        print!("step size: ");
        io::stdout().flush().unwrap_or_default();
        let mut step_sz = String::new();
        io::stdin()
            .read_line(&mut step_sz)
            .expect("failed to read line");

        if let Ok(x) = step_sz.trim().parse::<f32>() {
            step_size = x;
            break;
        } else {
            eprintln!("unable to parse step size");
        }
    }

    let points = plot(eq.trim(), x_min, x_max, step_size);

    if let Ok(points) = points {
        Chart::new(120, 60, x_min, x_max)
            .lineplot(&Shape::Lines(&points))
            .display();
    } else {
        eprintln!("{}", points.unwrap_err());
    }
}
