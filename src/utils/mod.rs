use std::io::{self, Write};

pub(crate) fn read_line() -> String {
    print!("> ");
    io::stdout().flush().unwrap_or_default();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");

    String::from(line.trim())
}
