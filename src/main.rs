use textplots::{Chart, Plot, Shape};

use rusty_maths::equation_analyzer::calculator::{calculate, plot};
use std::io::{self, Write};
use std::process::exit;

mod repl;
use repl::PreviousAnswer;
use repl::Repl;

fn main() {
    println!("Math!\n");

    let mut repl = Repl {
        previous_answer: 0.0,
        previous_answer_valid: false,
    };

    loop {
        print!("> ");
        io::stdout().flush().unwrap_or_default();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let line_trim = line.trim();

        if line_trim.is_empty() {
            continue;
        }

        if let Some(stripped) = line_trim.strip_prefix(':') {
            run_command(stripped, &mut repl);
            continue;
        }

        let ans_requested = line_trim.contains("ans");

        if repl.previous_answer_valid && ans_requested {
            let new_line = &line_trim.replace("ans", &repl.previous_answer.to_string());
            run(new_line, &mut repl);
        } else if !repl.previous_answer_valid && ans_requested {
            eprintln!("Invalid use of ans");
            continue;
        } else {
            run(line_trim, &mut repl);
        }
    }
}

fn run(line: &str, repl: &mut Repl) {
    let val = calculate(line);
    if let Ok(v) = val {
        repl.previous_answer(v, true);
        println!("{}", v);
    } else {
        repl.previous_answer(0.0, false);
        eprintln!("{}", val.unwrap_err());
    }
}

fn run_command(line: &str, repl: &mut Repl) {
    match line {
        "q" => exit(0),
        "p" => p(),
        _ => {
            eprintln!("Invalid Command");
            repl.previous_answer(0.0, false);
        }
    }
}

fn p() {
    print!("Equation: ");
    io::stdout().flush().unwrap_or_default();
    let mut eq = String::new();
    io::stdin().read_line(&mut eq).expect("Failed to read line");

    print!("x min: ");
    io::stdout().flush().unwrap_or_default();
    let mut x_min = String::new();
    io::stdin()
        .read_line(&mut x_min)
        .expect("Failed to read line");

    print!("x max: ");
    io::stdout().flush().unwrap_or_default();
    let mut x_max = String::new();
    io::stdin()
        .read_line(&mut x_max)
        .expect("Failed to read line");

    print!("step size: ");
    io::stdout().flush().unwrap_or_default();
    let mut step_size = String::new();
    io::stdin()
        .read_line(&mut step_size)
        .expect("Failed to read line");

    let points = plot(
        eq.trim(),
        x_min.trim().parse::<f32>().unwrap(),
        x_max.trim().parse::<f32>().unwrap(),
        step_size.trim().parse::<f32>().unwrap(),
    );

    if let Ok(points) = points {
        Chart::default().lineplot(&Shape::Lines(&points)).display();
    }
}
