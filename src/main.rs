mod modules;
use modules::logger::{Logger, StdoutLogger};
use modules::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let l = &mut StdoutLogger;

    match args.len() {
        1 => run::as_repl(l),
        2.. => run::as_cli_tool(&args, l),
        _ => l.eprint("invalid use of rmr"),
    }
}
