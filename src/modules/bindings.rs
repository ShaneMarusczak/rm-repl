//! `let` bindings: the one mechanism for naming things in the repl.
//!
//! `let a = 3` binds a value — the right side evaluates eagerly (against
//! the current bindings, so `let k = ans` works) and the *result* is
//! stored. `let g(x) = 2x^2` binds a function — the body is stored as
//! source and resolves late: it sees other bindings as they stand when the
//! function is called.
//!
//! Bindings persist to a file (one `let` line each, in definition order)
//! that replays at startup. `ans` is a binding too — auto-maintained after
//! every successful evaluation — but is never persisted and can't be bound
//! or removed by hand.

use rusty_maths::equation_analyzer::{calculator::calculate_with, Definition};
use std::fmt::Write;
use std::path::PathBuf;

use crate::modules::{error_render, logger::Logger, repl::Repl};

/// How a `let` line reached us: typed at the prompt (echoed above, errors
/// caret into it) or replayed from the persistence file (not on screen,
/// errors reprint the line and never abort the rest of the file).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum LetSource {
    Interactive,
    Replay,
}

pub(crate) fn is_let_line(line: &str) -> bool {
    line == "let" || line.starts_with("let ")
}

/// Does this non-`let` line look like an attempted binding (`a = 3`,
/// `g(x) = 2x^2`)? Used for the "start with let" hint. `x`/`y` on the left
/// are equation syntax (`y = 2x`), not bindings.
pub(crate) fn looks_like_binding(line: &str) -> bool {
    let Some((lhs, _)) = line.split_once('=') else {
        return false;
    };
    let lhs = lhs.trim();
    let name = lhs.split_once('(').map_or(lhs, |(n, _)| n.trim_end());
    if name == "x" || name == "y" {
        return false;
    }
    let mut chars = name.chars();
    chars.next().is_some_and(char::is_alphabetic)
        && chars.all(|c| c.is_alphabetic() || c.is_ascii_digit())
}

/// Handles a full `let` line (already trimmed). Returns whether a binding
/// was defined.
pub(crate) fn handle_let(
    line: &str,
    repl: &mut Repl,
    l: &mut impl Logger,
    source: LetSource,
) -> bool {
    match parse_and_define(line, repl) {
        Ok(notice) => {
            if source == LetSource::Interactive {
                l.print(&notice);
                save(repl, l);
            }
            true
        }
        Err(rendered) => {
            match source {
                LetSource::Interactive => l.eprint(&rendered.render_at_prompt()),
                LetSource::Replay => l.eprint(&rendered.render_with_source(line)),
            }
            false
        }
    }
}

/// A `let` failure, carrying enough position info to caret either the
/// echoed prompt line or a reprint of it.
struct LetError {
    message: String,
    /// Char range within the full `let` line, when the error is locatable.
    span: Option<(usize, usize)>,
    /// Body context for call-time-style errors (`in g(x) = body`); boxed to
    /// keep the common Err path small.
    in_function: Option<Box<BodyContext>>,
}

/// The function name, its body source, and the engine error whose span
/// refers to that body.
struct BodyContext {
    name: String,
    body: String,
    err: rusty_maths::equation_analyzer::EquationError,
}

impl LetError {
    fn plain(message: impl Into<String>) -> Self {
        LetError {
            message: message.into(),
            span: None,
            in_function: None,
        }
    }

    fn spanned(message: impl Into<String>, start: usize, end: usize) -> Self {
        LetError {
            message: message.into(),
            span: Some((start, end)),
            in_function: None,
        }
    }

    fn render_at_prompt(&self) -> String {
        if let Some(ctx) = &self.in_function {
            return error_render::format_error_in_function(&ctx.name, &ctx.body, &ctx.err);
        }
        match self.span {
            Some((start, end)) => {
                let pad = " ".repeat(error_render::REPL_PROMPT_WIDTH + start);
                let carets = "^".repeat((end - start).max(1));
                format!(
                    "{pad}{}{carets}{}\n{}",
                    error_render::CARET_START,
                    error_render::CARET_END,
                    self.message
                )
            }
            None => self.message.clone(),
        }
    }

    fn render_with_source(&self, line: &str) -> String {
        if let Some(ctx) = &self.in_function {
            return error_render::format_error_in_function(&ctx.name, &ctx.body, &ctx.err);
        }
        match self.span {
            Some((start, end)) => {
                let pad = " ".repeat(start);
                let carets = "^".repeat((end - start).max(1));
                format!(
                    "{line}\n{pad}{}{carets}{}\n{}",
                    error_render::CARET_START,
                    error_render::CARET_END,
                    self.message
                )
            }
            None => format!("{line}\n{}", self.message),
        }
    }
}

/// Parses `let name = expr` / `let name(x) = body`, defines the binding,
/// and returns the success notice.
fn parse_and_define(line: &str, repl: &mut Repl) -> Result<String, LetError> {
    const USAGE: &str = "Usage: let <name> = <expression>  or  let <name>(x) = <body>";

    let rest = line.strip_prefix("let").unwrap_or(line);
    if rest.trim().is_empty() {
        return Err(LetError::plain(USAGE));
    }

    let Some(eq_char_pos) = line.chars().position(|c| c == '=') else {
        return Err(LetError::plain(USAGE));
    };

    // Split on the first '=' by char position (spans are char-indexed).
    let lhs: String = line.chars().take(eq_char_pos).collect();
    let rhs: String = line.chars().skip(eq_char_pos + 1).collect();

    let lhs_trimmed = lhs["let".len()..].trim();
    let rhs_trimmed = rhs.trim();
    // Char offset of the trimmed right-hand side within the full line.
    let rhs_offset = eq_char_pos + 1 + leading_whitespace_chars(&rhs);

    if rhs_trimmed.is_empty() {
        return Err(LetError::plain(USAGE));
    }

    let (name, is_function) = match lhs_trimmed.split_once('(') {
        Some((name, params)) => {
            let name = name.trim_end();
            match params.trim_end().strip_suffix(')').map(str::trim) {
                Some("x") => (name, true),
                Some(other) => {
                    return Err(LetError::plain(format!(
                        "Function parameters must be exactly '(x)', got '({other})'"
                    )));
                }
                None => return Err(LetError::plain(USAGE)),
            }
        }
        None => (lhs_trimmed, false),
    };

    if name == "ans" {
        return Err(LetError::plain(
            "'ans' is set automatically after each calculation and can't be bound by hand",
        ));
    }

    let previous = describe(repl, name);

    if is_function {
        define_function(name, rhs_trimmed, rhs_offset, repl)?;
        Ok(notice(format!("{name}(x) = {rhs_trimmed}"), previous))
    } else {
        let value = define_value(name, rhs_trimmed, rhs_offset, repl)?;
        Ok(notice(format!("{name} = {value}"), previous))
    }
}

fn define_value(
    name: &str,
    rhs: &str,
    rhs_offset: usize,
    repl: &mut Repl,
) -> Result<f32, LetError> {
    let value =
        calculate_with(rhs, &repl.defs).map_err(|e| from_equation_error(e, rhs_offset, repl))?;

    if !value.is_finite() {
        return Err(LetError::plain(format!(
            "Cannot bind '{name}' to {value} — the expression must produce a finite number"
        )));
    }

    repl.defs
        .define_value(name, value)
        .map_err(|e| LetError::plain(e.message))?;
    Ok(value)
}

fn define_function(
    name: &str,
    body: &str,
    body_offset: usize,
    repl: &mut Repl,
) -> Result<(), LetError> {
    // `ans` changes on every evaluation, so a body capturing it would
    // either shift meaning constantly or lie about its stored source.
    if let Some(pos) = find_word(body, "ans") {
        let start = body_offset + pos;
        return Err(LetError::spanned(
            "'ans' changes with every evaluation — bind it first: let k = ans",
            start,
            start + "ans".len(),
        ));
    }

    // Snapshot whatever `name` currently means so a broken redefinition
    // can't destroy a working binding.
    let old = repl
        .defs
        .function_body(name)
        .map(str::to_string)
        .map(Restore::Function)
        .or_else(|| repl.defs.value(name).map(Restore::Value));

    repl.defs
        .define_function(name, body)
        .map_err(|e| LetError::plain(e.message))?;

    if let Err(e) = repl.defs.validate_function(name) {
        match old {
            Some(Restore::Function(b)) => {
                let _ = repl.defs.define_function(name, &b);
            }
            Some(Restore::Value(v)) => {
                let _ = repl.defs.define_value(name, v);
            }
            None => {
                repl.defs.undefine(name);
            }
        }
        return Err(LetError {
            message: e.message.clone(),
            span: None,
            in_function: Some(Box::new(BodyContext {
                name: name.to_string(),
                body: body.to_string(),
                err: e,
            })),
        });
    }
    Ok(())
}

enum Restore {
    Value(f32),
    Function(String),
}

/// Maps an engine error onto the typed `let` line: plain spans shift by
/// where the expression sits in the line; body-tagged errors keep their
/// body-relative span and render through the body reprint.
fn from_equation_error(
    e: rusty_maths::equation_analyzer::EquationError,
    offset: usize,
    repl: &Repl,
) -> LetError {
    if let Some(fn_name) = &e.in_function {
        let body = repl
            .defs
            .function_body(fn_name)
            .unwrap_or_default()
            .to_string();
        return LetError {
            message: e.message.clone(),
            span: None,
            in_function: Some(Box::new(BodyContext {
                name: fn_name.clone(),
                body,
                err: e,
            })),
        };
    }
    match e.span {
        Some(span) => LetError::spanned(e.message, offset + span.start, offset + span.end),
        None => LetError::plain(e.message),
    }
}

fn notice(binding: String, previous: Option<String>) -> String {
    match previous {
        Some(old) => format!("{binding}  (was {old})"),
        None => binding,
    }
}

/// A short description of what `name` is currently bound to, if anything.
fn describe(repl: &Repl, name: &str) -> Option<String> {
    if let Some(v) = repl.defs.value(name) {
        return Some(v.to_string());
    }
    repl.defs
        .function_body(name)
        .map(|body| format!("{name}(x) = {body}"))
}

pub(crate) fn undefine(name: &str, repl: &mut Repl, l: &mut impl Logger) {
    let name = name.trim();
    if name.is_empty() {
        l.eprint("Usage: :undef <name>");
        return;
    }
    if name == "ans" {
        l.eprint("'ans' is maintained automatically and can't be removed");
        return;
    }
    if repl.defs.undefine(name) {
        l.print(&format!("{name} removed"));
        save(repl, l);
    } else {
        l.eprint(&format!("Nothing named '{name}' is defined"));
    }
}

/// Where bindings live: `~/.rmr_bindings`, one `let` line per binding.
pub(crate) fn default_bindings_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".rmr_bindings"))
}

/// Replays the persistence file. Broken lines warn and are skipped — the
/// rest of the file still loads.
pub(crate) fn load(repl: &mut Repl, l: &mut impl Logger) {
    let Some(path) = repl.bindings_path.clone() else {
        return;
    };
    let Ok(content) = std::fs::read_to_string(&path) else {
        return; // no file yet — first run
    };
    for (i, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        let ok = is_let_line(line) && handle_let(line, repl, l, LetSource::Replay);
        if !ok {
            l.eprint(&format!(
                "Warning: ignored line {} of {}: {line}",
                i + 1,
                path.display()
            ));
        }
    }
}

/// Rewrites the persistence file from the current bindings. `ans` is
/// session state, not a binding the user made — it never persists.
pub(crate) fn save(repl: &Repl, l: &mut impl Logger) {
    let Some(path) = &repl.bindings_path else {
        return;
    };
    let mut out = String::new();
    for def in repl.defs.iter() {
        // Writing to String never fails, safe to ignore
        match def {
            Definition::Value { name, value } => {
                if name != "ans" {
                    let _ = writeln!(out, "let {name} = {value}");
                }
            }
            Definition::Function { name, body } => {
                let _ = writeln!(out, "let {name}(x) = {body}");
            }
        }
    }
    if let Err(e) = std::fs::write(path, out) {
        l.eprint(&format!(
            "Warning: could not save bindings to {}: {e}",
            path.display()
        ));
    }
}

fn leading_whitespace_chars(s: &str) -> usize {
    s.chars().take_while(|c| c.is_whitespace()).count()
}

/// Char index of `needle` in `haystack` where neither neighbor is
/// alphanumeric — i.e. as a standalone word.
fn find_word(haystack: &str, needle: &str) -> Option<usize> {
    let chars: Vec<char> = haystack.chars().collect();
    let target: Vec<char> = needle.chars().collect();
    if target.is_empty() || chars.len() < target.len() {
        return None;
    }
    for i in 0..=(chars.len() - target.len()) {
        if chars[i..i + target.len()] != target[..] {
            continue;
        }
        let prev_ok = i == 0 || !chars[i - 1].is_alphanumeric();
        let next_ok = i + target.len() == chars.len() || !chars[i + target.len()].is_alphanumeric();
        if prev_ok && next_ok {
            return Some(i);
        }
    }
    None
}
