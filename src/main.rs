use logger::Logger;

mod commands;
mod evaluate;
mod graphing;
mod inputs;
mod logger;
mod repl;
mod run;
mod string_maker;
mod structs;
mod tests;
mod variables;

use crate::logger::StdoutLogger;

//TODO: scrollable graph, like iteractive graph but you move a point along the graph instead of moving the graph
//left right moves the point, up down switches graphs (if multiple)

//TODO: also, UNIT TESTS

//TODO: write a proper readme

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let l = &mut StdoutLogger;

    match args.len() {
        1 => run::as_repl(l),
        2.. => run::as_cli_tool(&args, l),
        _ => l.eprint("invalid use of rmr"),
    }
}
