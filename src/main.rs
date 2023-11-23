mod commands;
mod evaluate;
mod graphing;
mod inputs;
mod repl;
mod run;
mod string_maker;
mod structs;
mod variables;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run::as_repl(),
        2.. => run::as_cli_tool(&args),
        _ => eprintln!("invalid use of rmr"),
    }
}
