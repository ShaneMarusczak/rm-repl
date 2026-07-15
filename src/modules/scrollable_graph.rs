//! `:sg` — scrollable graph. The window stays fixed while a cursor walks
//! the curve, with a live coordinate readout above the frame:
//!
//! ```text
//! y = sin(x^2) │ x: 1.25  y: 0.95
//! ┌────────────────────────┐1.50
//! │      ⡴⠋⠈●⡆    ⢀⡖⠑⡄      │
//! ...
//! ```
//!
//! Left/Right move the cursor one braille column of x at a time, Up/Down
//! switch between `|`-separated equations, `q` quits. The marker is drawn
//! *onto* the rendered graph — character surgery after rasterization —
//! because a cursor should stand apart from the data, unlike the axes,
//! which live in the cell matrix and blend with it.

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, ExecutableCommand};
use rusty_maths::equation_analyzer::{calculator::plot_with, Definitions};

use crate::modules::{
    common::GraphOptions, error_render, graphing, inputs::get_g_inputs, logger::Logger,
};

const MARKER: char = '●';

/// The fixed view a session scrolls within: the rendered base graph plus
/// the world-to-cell mapping parameters it was rendered with.
struct View {
    base: String,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    width: usize,
    height: usize,
}

pub(crate) fn sg(l: &mut impl Logger, go: &GraphOptions, defs: &Definitions, precision: usize) {
    let (eq, x_min, x_max) = get_g_inputs(l);

    let (base, y_min, y_max) = match graphing::graph_with_view(&eq, x_min, x_max, go, defs) {
        Ok(v) => v,
        Err(e) => {
            l.eprint(&error_render::render_error_with_source(&eq, &e, defs));
            return;
        }
    };
    let view = View {
        base,
        x_min,
        x_max,
        y_min,
        y_max,
        width: go.width,
        height: go.height,
    };

    let eqs: Vec<&str> = eq.split('|').map(str::trim).collect();
    let mut active = 0usize;
    let mut cursor_x = (x_min + x_max) / 2.0;
    // One braille character column of x per keypress.
    let x_step = (x_max - x_min) / view.width as f32 * 2.0;

    let mut stdout = std::io::stdout();
    let mut drawn = draw(l, &view, &eqs, active, cursor_x, defs, precision);

    // Enable raw mode for key capture - ignore errors, continue without
    // interactive mode (matches :ig).
    let _ = enable_raw_mode();
    loop {
        let Ok(Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            ..
        })) = read()
        else {
            continue;
        };

        match code {
            KeyCode::Left => cursor_x = (cursor_x - x_step).max(view.x_min),
            KeyCode::Right => cursor_x = (cursor_x + x_step).min(view.x_max),
            KeyCode::Up => active = (active + 1) % eqs.len(),
            KeyCode::Down => active = (active + eqs.len() - 1) % eqs.len(),
            KeyCode::Char('q') => break,
            _ => continue,
        }

        let _ = disable_raw_mode();
        let _ = stdout.execute(cursor::MoveUp(drawn as u16));
        drawn = draw(l, &view, &eqs, active, cursor_x, defs, precision);
        let _ = enable_raw_mode();
    }
    let _ = disable_raw_mode();
}

/// Prints the readout line and the marked graph; returns how many terminal
/// lines that was, so the caller can move back up over them.
fn draw(
    l: &mut impl Logger,
    view: &View,
    eqs: &[&str],
    active: usize,
    cursor_x: f32,
    defs: &Definitions,
    precision: usize,
) -> usize {
    let y = eval_at(eqs[active], cursor_x, defs);

    let which = if eqs.len() > 1 {
        format!("[{}/{}] ", active + 1, eqs.len())
    } else {
        String::new()
    };
    let y_text = match y {
        Some(v) => format!("{v:.precision$}"),
        None => "undefined".to_string(),
    };
    let readout = format!(
        "{which}{} │ x: {cursor_x:.precision$}  y: {y_text}",
        eqs[active]
    );
    // Pad to the frame width so a shorter readout fully overwrites the
    // previous one during in-place redraws.
    let frame_width = view.width / 2 + 2;
    l.print(&format!("{readout:<frame_width$}"));

    let marked = overlay_marker(&view.base, marker_cell(cursor_x, y, view));
    l.print(&marked);

    1 + marked.lines().count()
}

/// The y value of one equation at one x, or None where it doesn't evaluate
/// to a plottable number.
fn eval_at(eq: &str, x: f32, defs: &Definitions) -> Option<f32> {
    plot_with(eq, x, x, 1.0, defs)
        .ok()
        .and_then(|points| points.first().map(|p| p.y))
        .filter(|y| y.is_finite())
}

/// Maps a world-space cursor position onto a braille character cell —
/// `(row, col)` within the graph's braille grid, row 0 at the top. The
/// same normalization the plotter uses, at character resolution; y is
/// clamped so an off-view cursor pins to the frame edge.
fn marker_cell(cursor_x: f32, y: Option<f32>, view: &View) -> Option<(usize, usize)> {
    let y = y?;
    let char_rows = view.height / 4;
    let char_cols = view.width / 2;
    if char_rows == 0 || char_cols == 0 {
        return None;
    }

    let fx = ((cursor_x - view.x_min) / (view.x_max - view.x_min)).clamp(0.0, 1.0);
    let fy = ((y - view.y_min) / (view.y_max - view.y_min)).clamp(0.0, 1.0);

    let col = ((fx * view.width as f32) as usize / 2).min(char_cols - 1);
    let row_from_bottom = ((fy * view.height as f32) as usize / 4).min(char_rows - 1);
    let row = char_rows - 1 - row_from_bottom;

    Some((row, col))
}

/// Replaces one braille cell of the rendered graph with the marker.
/// `cell` is (row, col) in the braille grid; the +1 offsets skip the box
/// border. `None` leaves the graph untouched.
fn overlay_marker(base: &str, cell: Option<(usize, usize)>) -> String {
    let Some((row, col)) = cell else {
        return base.to_string();
    };

    let mut lines: Vec<String> = base.lines().map(str::to_string).collect();
    if let Some(line) = lines.get_mut(row + 1) {
        let target = col + 1;
        *line = line
            .chars()
            .enumerate()
            .map(|(i, c)| if i == target { MARKER } else { c })
            .collect();
    }
    lines.join("\n")
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    /// Test-only doorway to the pure marker math.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn marker_cell_for(
        cursor_x: f32,
        y: Option<f32>,
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        marker_cell(
            cursor_x,
            y,
            &View {
                base: String::new(),
                x_min,
                x_max,
                y_min,
                y_max,
                width,
                height,
            },
        )
    }

    pub(crate) fn overlay(base: &str, cell: Option<(usize, usize)>) -> String {
        overlay_marker(base, cell)
    }
}
