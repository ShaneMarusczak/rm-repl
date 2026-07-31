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

/// Narrowest usable graph: one braille glyph spans 2×4 cells and height is
/// width/2, so anything under 8 renders zero glyph rows.
pub(crate) const MIN_GRAPH_WIDTH: usize = 8;

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

    /// Applies a new graph width (height tracks at width/2). Returns false,
    /// leaving dimensions unchanged, if `width` is below [`MIN_GRAPH_WIDTH`].
    pub(crate) fn update_dimensions(&mut self, width: usize) -> bool {
        if width < MIN_GRAPH_WIDTH {
            return false;
        }
        self.height = width / 2;
        self.width = width;
        true
    }

    /// Records a successful evaluation's result as the `ans` binding.
    pub(crate) fn set_ans(&mut self, value: f32) {
        // "ans" is a valid non-catalog name, so this cannot fail.
        let _ = self.defs.define_value("ans", value);
    }
}
