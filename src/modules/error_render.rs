use rusty_maths::equation_analyzer::{Definitions, EquationError};

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
/// several prompts after the equation) or came from argv rather than an
/// echoed prompt line.
pub(crate) fn format_error_with_source(source: &str, err: &EquationError) -> String {
    if err.span.is_some() {
        format!("{source}\n{}", format_error(err, Some(0)))
    } else {
        err.to_string()
    }
}

/// Renders an error that arose inside a user-defined function's body at
/// call time: reprints the definition and carets into the *body*, since
/// the error's span refers to the body source, not the typed line.
///
/// ```text
/// in g(x) = a * x
///           ^
/// Unknown name 'a'
/// ```
pub(crate) fn format_error_in_function(name: &str, body: &str, err: &EquationError) -> String {
    let header = format!("in {name}(x) = {body}");
    match err.span {
        Some(span) => {
            let indent = format!("in {name}(x) = ").chars().count() + span.start;
            let carets = "^".repeat(span.len().max(1));
            format!(
                "{header}\n{}{CARET_START}{carets}{CARET_END}\n{}",
                " ".repeat(indent),
                err.message
            )
        }
        None => format!("in {name}(x): {}", err.message),
    }
}

/// The one-stop renderer for evaluation errors when bindings are in scope:
/// body-tagged errors reprint the offending definition; everything else
/// carets under the echoed prompt line.
pub(crate) fn render_repl_error(err: &EquationError, defs: &Definitions) -> String {
    render_tagged(err, defs).unwrap_or_else(|| format_error(err, Some(REPL_PROMPT_WIDTH)))
}

/// Like [`format_error_with_source`], but body-tagged errors reprint the
/// offending definition instead of the (irrelevant) equation text.
pub(crate) fn render_error_with_source(
    source: &str,
    err: &EquationError,
    defs: &Definitions,
) -> String {
    render_tagged(err, defs).unwrap_or_else(|| format_error_with_source(source, err))
}

fn render_tagged(err: &EquationError, defs: &Definitions) -> Option<String> {
    let name = err.in_function.as_deref()?;
    let body = defs.function_body(name)?;
    Some(format_error_in_function(name, body, err))
}
