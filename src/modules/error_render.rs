use rusty_maths::equation_analyzer::EquationError;

// Visible width of the REPL prompt (`>> `) set in run::build_interface.
pub(crate) const REPL_PROMPT_WIDTH: usize = 3;

pub(crate) const CARET_START: &str = "\u{001b}[31m";
pub(crate) const CARET_END: &str = "\u{001b}[0m";

/// Renders an `EquationError` for the terminal.
///
/// When `echo_indent` is `Some(n)`, the offending input is still visible on
/// the line above, starting at column `n` (the prompt width), so this emits
/// a red caret line aligned under it followed by the message:
///
/// ```text
/// >> 2 + foo(3)
///        ^^^
/// Invalid function name foo
/// ```
///
/// Without an indent (or without a span) it falls back to the error's
/// `Display` form, which includes the 1-based character position.
pub(crate) fn format_error(err: &EquationError, echo_indent: Option<usize>) -> String {
    match (err.span, echo_indent) {
        (Some(span), Some(indent)) => {
            let pad = " ".repeat(indent + span.start);
            let carets = "^".repeat(span.len().max(1));
            format!("{pad}{CARET_START}{carets}{CARET_END}\n{}", err.message)
        }
        _ => err.to_string(),
    }
}

/// Reprints the offending input and points at it — for contexts where the
/// echoed input has scrolled away (the `:g`/`:t` sub-prompt flows print
/// several prompts after the equation) or where the evaluated text differs
/// from what was typed (`ans`/variable substitution).
pub(crate) fn format_error_with_source(source: &str, err: &EquationError) -> String {
    if err.span.is_some() {
        format!("{source}\n{}", format_error(err, Some(0)))
    } else {
        err.to_string()
    }
}
