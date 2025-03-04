pub(crate) trait Logger {
    fn print(&mut self, value: &str);
    fn eprint(&mut self, value: &str);
}

pub(crate) struct StdoutLogger;

impl Logger for StdoutLogger {
    fn print(&mut self, value: &str) {
        println!("{}", value);
    }
    fn eprint(&mut self, value: &str) {
        eprintln!("{}", value);
    }
}
