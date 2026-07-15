//! Tab completion for the repl: `:commands`, catalog names, and the
//! user's own `let` bindings.
//!
//! Completion candidates come from three places — the command list here,
//! the engine's catalog, and a snapshot of the current bindings that the
//! repl loop refreshes before each read. Functions complete with a
//! trailing `(`, `log` with its `_` base syntax, values and constants
//! bare.

use linefeed::complete::{Completer, Completion, Suffix};
use linefeed::prompter::Prompter;
use linefeed::terminal::Terminal;
use rusty_maths::equation_analyzer::{
    catalog::{self, SymbolKind},
    Definition, Definitions,
};
use std::sync::Mutex;

/// Word-break characters for the repl: whitespace, operators, and
/// delimiters. `:` is deliberately not a break so `:gr<TAB>` completes as
/// one word.
pub(crate) const WORD_BREAK_CHARS: &str = " \t+-*/^%(),=!|<>";

/// Every repl command, with whether it takes an argument (those complete
/// with a trailing space).
const COMMANDS: &[(&str, bool)] = &[
    (":g", false),
    (":graph", false),
    (":t", false),
    (":table", false),
    (":o", false),
    (":ag", false),
    (":ig", false),
    (":sg", false),
    (":la", false),
    (":c", false),
    (":cube", false),
    (":3d", false),
    (":qbc", false),
    (":cbc", false),
    (":p", true),
    (":precision", true),
    (":fns", false),
    (":functions", false),
    (":undef", true),
    (":clear", false),
    (":h", false),
    (":help", false),
    (":q", false),
    (":quit", false),
];

pub(crate) struct RmrCompleter {
    /// `(name, is_function)` for the current bindings; refreshed by the
    /// repl loop via [`sync`](Self::sync) before each prompt.
    bindings: Mutex<Vec<(String, bool)>>,
}

impl RmrCompleter {
    pub(crate) fn new() -> Self {
        RmrCompleter {
            bindings: Mutex::new(Vec::new()),
        }
    }

    /// Snapshots the current definitions (including `ans` once it exists).
    pub(crate) fn sync(&self, defs: &Definitions) {
        let names = defs
            .iter()
            .map(|d| match d {
                Definition::Value { name, .. } => (name.to_string(), false),
                Definition::Function { name, .. } => (name.to_string(), true),
            })
            .collect();
        if let Ok(mut bindings) = self.bindings.lock() {
            *bindings = names;
        }
    }
}

impl<Term: Terminal> Completer<Term> for RmrCompleter {
    fn complete(
        &self,
        word: &str,
        prompter: &Prompter<Term>,
        start: usize,
        _end: usize,
    ) -> Option<Vec<Completion>> {
        let before = &prompter.buffer()[..start];
        let bindings = self.bindings.lock().ok()?;
        let found = candidates(word, before, &bindings);
        if found.is_empty() {
            return None;
        }
        Some(
            found
                .into_iter()
                .map(|(text, suffix)| Completion {
                    completion: text,
                    display: None,
                    suffix: match suffix {
                        Some(c) => Suffix::Some(c),
                        None => Suffix::None,
                    },
                })
                .collect(),
        )
    }
}

/// The completions for `word`, given the text `before` it on the line and
/// the current bindings. Returned as `(text, optional suffix char)`,
/// sorted; pure so it's testable without a terminal.
pub(crate) fn candidates(
    word: &str,
    before: &str,
    bindings: &[(String, bool)],
) -> Vec<(String, Option<char>)> {
    let mut out: Vec<(String, Option<char>)> = Vec::new();

    // Completing the command itself.
    if word.starts_with(':') {
        for (cmd, takes_arg) in COMMANDS {
            if cmd.starts_with(word) {
                out.push(((*cmd).to_string(), takes_arg.then_some(' ')));
            }
        }
        out.sort();
        return out;
    }

    let context = before.trim();

    // `:undef <name>` — only bindings can be removed.
    if context == ":undef" {
        for (name, _) in bindings {
            if name.starts_with(word) {
                out.push((name.clone(), None));
            }
        }
        out.sort();
        return out;
    }

    // `:fns <name>` — name lookup, so no call-syntax suffixes.
    if context == ":fns" || context == ":functions" {
        for name in catalog_names() {
            if name.starts_with(word) {
                out.push((name.to_string(), None));
            }
        }
        for (name, _) in bindings {
            if name.starts_with(word) {
                out.push((name.clone(), None));
            }
        }
        out.sort();
        return out;
    }

    // An expression: catalog entries and bindings, functions opening their
    // call; `let` offered at the start of a line.
    if before.trim().is_empty() && "let".starts_with(word) && !word.is_empty() {
        out.push(("let".to_string(), Some(' ')));
    }
    for sym in catalog::all() {
        let suffix = match sym.kind {
            SymbolKind::Unary(_) | SymbolKind::UnaryChecked(_) | SymbolKind::Variadic { .. } => {
                Some('(')
            }
            SymbolKind::LogBase => Some('_'),
            // Word operators (`mod`) complete bare; the variable `x` and
            // glyph operators aren't worth a keystroke.
            SymbolKind::Constant(_) | SymbolKind::Operator { .. } => None,
            SymbolKind::Variable => continue,
        };
        for name in std::iter::once(sym.name).chain(sym.aliases.iter().copied()) {
            if identifier_like(name) && name.starts_with(word) {
                out.push((name.to_string(), suffix));
            }
        }
    }
    for (name, is_function) in bindings {
        if name.starts_with(word) {
            out.push((name.clone(), is_function.then_some('(')));
        }
    }
    out.sort();
    out
}

/// Catalog labels that lex as identifiers (skips operator glyphs, keeps
/// word operators like `mod`).
fn catalog_names() -> impl Iterator<Item = &'static str> {
    catalog::all()
        .iter()
        .filter(|s| !matches!(s.kind, SymbolKind::Variable))
        .flat_map(|s| std::iter::once(s.name).chain(s.aliases.iter().copied()))
        .filter(|name| identifier_like(name))
}

/// Typeable-as-a-word names only — `mod` yes, `%%` and `|>` no.
fn identifier_like(name: &str) -> bool {
    name.chars().next().is_some_and(char::is_alphabetic)
}
