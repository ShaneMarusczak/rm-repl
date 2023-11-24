pub(crate) trait Logger {
    fn print(&mut self, value: &str);
    fn eprint(&mut self, value: &str);
}

pub(crate) struct StdoutLogger;
pub(crate) struct TestLogger {
    pub(crate) val: String,
    pub(crate) error_val: String,
}

impl Logger for StdoutLogger {
    fn print(&mut self, value: &str) {
        println!("{}", value);
    }
    fn eprint(&mut self, value: &str) {
        eprintln!("{}", value);
    }
}

impl Logger for TestLogger {
    fn print(&mut self, value: &str) {
        self.val = value.to_owned();
    }
    fn eprint(&mut self, value: &str) {
        self.error_val = value.to_owned();
    }
}
