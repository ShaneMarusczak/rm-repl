use rusty_maths::equation_analyzer::Definitions;
use std::path::PathBuf;

pub(crate) struct Repl {
    /// User `let` bindings — named values and functions — plus the
    /// auto-maintained `ans` value. Evaluation runs against this set.
    pub(crate) defs: Definitions,

    pub(crate) height: usize,
    pub(crate) width: usize,
    pub(crate) precision: usize,

    /// Where bindings persist across sessions; `None` disables persistence
    /// (tests, or no resolvable home directory).
    pub(crate) bindings_path: Option<PathBuf>,
}

//the Repl object is a user session object
//store results of math problems
//give a session cache, debug flags

impl Repl {
    pub(crate) fn new(width: usize) -> Self {
        Self {
            defs: Definitions::new(),
            height: width / 2,
            width,
            precision: 2,
            bindings_path: None,
        }
    }

    pub(crate) fn update_dimensions(&mut self, width: usize) {
        self.height = width / 2;
        self.width = width;
    }

    /// Records a successful evaluation's result as the `ans` binding.
    pub(crate) fn set_ans(&mut self, value: f32) {
        // "ans" is a valid non-catalog name, so this cannot fail.
        let _ = self.defs.define_value("ans", value);
    }
}
