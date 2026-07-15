use rusty_maths::{
    equation_analyzer::calculator::plot_with,
    equation_analyzer::catalog::{self, Category, Symbol, SymbolKind},
    equation_analyzer::{Definition, Definitions},
    linear_algebra::{vector_mean, vector_sum},
};

use crate::modules::{
    bindings,
    common::*,
    cube::cube,
    error_render,
    graphing::graph,
    inputs::{get_g_inputs, get_matrix_input, get_numerical_input},
    logger::Logger,
    repl::Repl,
    string_maker::make_table_string,
};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, ExecutableCommand};

use super::{
    bezier_curve::{cubic_bezier, quadratic_bezier},
    inputs::read_user_input,
};

pub(crate) fn run_command(line: &str, l: &mut impl Logger, repl: &mut Repl) {
    if let Some(n) = line
        .strip_prefix("p ")
        .or_else(|| line.strip_prefix("precision "))
    {
        set_precision(n, repl, l);
        return;
    }

    // `:fns <name>` — describe a single symbol. Bare `:fns` falls through
    // to the match below and prints the whole catalog.
    if let Some(name) = line.strip_prefix("fns ") {
        fns_one(name.trim(), l, &repl.defs);
        return;
    }

    if let Some(name) = line.strip_prefix("undef ") {
        bindings::undefine(name, repl, l);
        return;
    }

    let go = GraphOptions {
        y_min: -7.,
        y_max: 7.,
        width: repl.width,
        height: repl.height,
    };
    match line {
        //TODO: scrollable graph (sg), like iteractive graph but you move a point along the graph instead of moving the graph
        //left right moves the point, up down switches graphs (if multiple)

        //TODO: add math tutor (mt) option that starts a chat session with chat gpt
        //that inserts "you are a math tutor" or w/e to the start of each prompt

        //TODO: a fast forier transform (fft), takes a sound file, shows a report of all wave forms seen, with time ranges when heard
        "t" | "table" => t(l, &repl.defs),
        "g" | "graph" => g(l, &go, &repl.defs),
        "o" | "graph options" => gos(l, repl),
        "ag" | "animated graph" => ag(l, &go, &repl.defs),
        "ig" | "interactive graph" => ig(l, &go, &repl.defs),
        "la" | "linear algebra" => la(l),
        "c" | "cube" | "3d" => c(l, &go),
        "qbc" => qbc(l, &go),
        "cbc" => cbc(l, &go),
        "fns" | "functions" => fns_all(l, &repl.defs),
        "undef" => l.eprint("Usage: :undef <name>"),
        "h" | "help" => h(l),
        _ => {
            l.eprint(&format!("Invalid command '{line}'. Type ':h' for help."));
        }
    }
}

fn h(l: &mut impl Logger) {
    l.print("Available commands:");
    l.print(":g  | :graph -> graphing mode");
    l.print(":t  | :table -> table mode");
    l.print(":o  | :graph options -> graph options mode");
    l.print(":ag | :animated graph -> animated graph mode");
    l.print(":ig | :interactive graph -> interactive graph mode");
    l.print(":la | :linear algebra -> linear algebra mode");
    l.print(":c  | :cube | :3d -> renders an animated cube to the terminal");
    l.print(":qbc -> quadratic bezier curve");
    l.print(":cbc -> cubic bezier curve");
    l.print(":p  | :precision <n> -> set decimal precision (e.g. :p 4)");
    l.print(":fns [name] -> list every math function/operator/constant; with a name, show just that one");
    l.print(":undef <name> -> remove a let binding");
    l.print(":q  | :quit -> exits the repl session");
    l.print("");
    l.print("Bindings (persist across sessions):");
    l.print("let a = 3        -> bind a value (right side evaluates now)");
    l.print("let g(x) = 2x^2  -> bind a function (body sees bindings at call time)");
    l.print("ans              -> the last successful answer, usable in any expression");
}

fn kind_label(k: &SymbolKind) -> &'static str {
    match k {
        SymbolKind::Constant(_) => "constant",
        SymbolKind::Unary(_) | SymbolKind::UnaryChecked(_) => "function",
        SymbolKind::Variadic { .. } => "function",
        SymbolKind::LogBase => "function",
        SymbolKind::Operator { .. } => "operator",
        SymbolKind::Variable => "variable",
    }
}

fn category_label(c: Category) -> &'static str {
    match c {
        Category::Constant => "Constants",
        Category::Arithmetic => "Arithmetic",
        Category::Trig => "Trigonometric",
        Category::InverseTrig => "Inverse trig",
        Category::Hyperbolic => "Hyperbolic",
        Category::Logarithmic => "Logarithmic",
        Category::Statistical => "Statistical",
        Category::AngleConversion => "Angle conversion",
        Category::Piping => "Piping",
        Category::Variable => "Variables",
    }
}

/// Order in which categories are printed. Matches the source-level ordering
/// in catalog.rs so the whole listing reads top-to-bottom naturally.
const CATEGORY_ORDER: &[Category] = &[
    Category::Constant,
    Category::Trig,
    Category::InverseTrig,
    Category::Hyperbolic,
    Category::AngleConversion,
    Category::Arithmetic,
    Category::Logarithmic,
    Category::Statistical,
    Category::Piping,
    Category::Variable,
];

fn label_for(sym: &Symbol) -> String {
    if sym.aliases.is_empty() {
        sym.name.to_string()
    } else {
        format!("{} ({})", sym.name, sym.aliases.join(", "))
    }
}

fn fns_all(l: &mut impl Logger, defs: &Definitions) {
    // Compute the longest label so summaries line up in a column.
    let label_width = catalog::all()
        .iter()
        .map(|s| label_for(s).chars().count())
        .max()
        .unwrap_or(4)
        .max(4);

    l.print("");
    for cat in CATEGORY_ORDER {
        let mut printed_header = false;
        for sym in catalog::by_category(*cat) {
            if !printed_header {
                l.print(&format!("── {} ──", category_label(*cat)));
                printed_header = true;
            }
            let label = label_for(sym);
            let pad = label_width - label.chars().count();
            l.print(&format!("  {label}{}  {}", " ".repeat(pad), sym.summary));
        }
        if printed_header {
            l.print("");
        }
    }
    if !defs.is_empty() {
        l.print("── Your bindings ──");
        for def in defs.iter() {
            match def {
                Definition::Value { name, value } => l.print(&format!("  {name} = {value}")),
                Definition::Function { name, body } => l.print(&format!("  {name}(x) = {body}")),
            }
        }
        l.print("");
    }
    l.print("Use ':fns <name>' for details on any single symbol (e.g. ':fns atan2').");
}

fn fns_one(name: &str, l: &mut impl Logger, defs: &Definitions) {
    if name.is_empty() {
        l.eprint("Usage: :fns <name>   (or bare :fns to list everything)");
        return;
    }
    // Catalog names and binding names can't collide, so order is cosmetic.
    if let Some(v) = defs.value(name) {
        l.print("");
        l.print(&format!("  {name} = {v}"));
        l.print("  kind      value binding (yours — remove with :undef)");
        return;
    }
    if let Some(body) = defs.function_body(name) {
        l.print("");
        l.print(&format!("  {name}(x) = {body}"));
        l.print("  kind      function binding (yours — remove with :undef)");
        return;
    }
    match catalog::find(name) {
        Some(sym) => {
            l.print("");
            l.print(&format!("  {}", label_for(sym)));
            l.print(&format!("  kind      {}", kind_label(&sym.kind)));
            l.print(&format!("  category  {}", category_label(sym.category)));
            l.print(&format!("  summary   {}", sym.summary));
            l.print(&format!("  example   {}", sym.example));
        }
        None => l.eprint(&format!(
            "No symbol '{name}' — try ':fns' for the full list."
        )),
    }
}

fn set_precision(n: &str, repl: &mut Repl, l: &mut impl Logger) {
    match n.trim().parse::<usize>() {
        Ok(p) => {
            repl.precision = p;
            l.print(&format!("Precision set to {p}"));
        }
        Err(_) => l.eprint(&format!("'{n}' is not a valid precision value")),
    }
}

fn cbc(l: &mut impl Logger, go: &GraphOptions) {
    l.print(&format!(
        "Lower Right Coordinates - x:{}, y:{}",
        go.width, go.height
    ));

    let p1_x = get_numerical_input("start x: ", l);
    let p1_y = get_numerical_input("start y: ", l);

    let p2_x = get_numerical_input("control1 x: ", l);
    let p2_y = get_numerical_input("control1 y: ", l);

    let p3_x = get_numerical_input("control2 x: ", l);
    let p3_y = get_numerical_input("control2 y: ", l);

    let p4_x = get_numerical_input("end x: ", l);
    let p4_y = get_numerical_input("end y: ", l);

    let p1 = Point::new(p1_x, p1_y);
    let p2 = Point::new(p2_x, p2_y);
    let p3 = Point::new(p3_x, p3_y);
    let p4 = Point::new(p4_x, p4_y);

    cubic_bezier(p1, p2, p3, p4, go, l);
}

fn qbc(l: &mut impl Logger, go: &GraphOptions) {
    l.print(&format!(
        "Lower Right Coordinates - x:{}, y:{}",
        go.width, go.height
    ));
    let p1_x = get_numerical_input("start x: ", l);
    let p1_y = get_numerical_input("start y: ", l);

    let p2_x = get_numerical_input("control x: ", l);
    let p2_y = get_numerical_input("control y: ", l);

    let p3_x = get_numerical_input("end x: ", l);
    let p3_y = get_numerical_input("end y: ", l);

    let p1 = Point::new(p1_x, p1_y);
    let p2 = Point::new(p2_x, p2_y);
    let p3 = Point::new(p3_x, p3_y);

    quadratic_bezier(p1, p2, p3, go, l);
}

fn c(l: &mut impl Logger, go: &GraphOptions) {
    cube(l, go);
}

fn gos(l: &mut impl Logger, repl: &mut Repl) {
    repl.update_dimensions(get_numerical_input("width: ", l));
}

fn t(l: &mut impl Logger, defs: &Definitions) {
    let (eq, x_min, x_max) = get_g_inputs(l);
    let step_size = get_numerical_input("step size: ", l);

    let points = plot_with(&eq, x_min, x_max, step_size, defs);

    match points {
        Ok(rm_points) => {
            let points: Vec<_> = rm_points
                .into_iter()
                .map(|p| Point::new(p.x, p.y))
                .collect();
            l.print(&make_table_string(points));
        }
        // The equation prompt has scrolled past (x min/max and step size
        // prompts printed since), so reprint it and point at the error.
        Err(e) => l.eprint(&error_render::render_error_with_source(&eq, &e, defs)),
    }
}

fn g(l: &mut impl Logger, go: &GraphOptions, defs: &Definitions) {
    let (eq, x_min, x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go, defs);

    match g {
        Ok(g) => l.print(&g),
        Err(e) => l.eprint(&error_render::render_error_with_source(&eq, &e, defs)),
    }
}

fn ag(l: &mut impl Logger, go: &GraphOptions, defs: &Definitions) {
    let mut stdout = std::io::stdout();

    let (eq, x_min, x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go, defs);

    if let Ok(g) = g {
        l.print(&g);
        let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;

        for n in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(90));

            let _ = stdout.execute(cursor::MoveUp(new_lines as u16));

            if let Ok(g) = graph(&eq, x_min - n as f32, x_max + n as f32, go, defs) {
                l.print(&g);
            }
        }
    } else if let Err(e) = g {
        l.eprint(&error_render::render_error_with_source(&eq, &e, defs));
    }
}

fn ig(l: &mut impl Logger, go: &GraphOptions, defs: &Definitions) {
    let mut stdout = std::io::stdout();

    let (eq, mut x_min, mut x_max) = get_g_inputs(l);
    let g = graph(&eq, x_min, x_max, go, defs);

    if let Ok(g) = g {
        l.print(&g);

        let new_lines = g.chars().filter(|c| c.eq_ignore_ascii_case(&'\n')).count() + 1;
        // Enable raw mode for key capture - ignore errors, continue without interactive mode
        let _ = enable_raw_mode();

        loop {
            match read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                })) => {
                    let _ = disable_raw_mode();
                    x_min += 1.0;
                    x_max += 1.0;
                    let _ = stdout.execute(cursor::MoveUp(new_lines as u16));

                    if let Ok(g) = graph(&eq, x_min, x_max, go, defs) {
                        l.print(&g);
                    }

                    let _ = enable_raw_mode();
                }

                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                })) => {
                    let _ = disable_raw_mode();
                    x_min -= 1.0;
                    x_max -= 1.0;

                    let _ = stdout.execute(cursor::MoveUp(new_lines as u16));

                    if let Ok(g) = graph(&eq, x_min, x_max, go, defs) {
                        l.print(&g);
                    }

                    let _ = enable_raw_mode();
                }

                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                })) => break,
                _ => continue,
            }
        }
        let _ = disable_raw_mode();
    } else if let Err(e) = g {
        l.eprint(&error_render::render_error_with_source(&eq, &e, defs));
    }
}

fn la(l: &mut impl Logger) {
    loop {
        if let Ok(op_code) = read_user_input("operation: ") {
            match op_code.as_str() {
                "vs" | "vector sum" => {
                    let m = get_matrix_input(l);
                    let sum = vector_sum(&m);
                    l.print(&format!("{sum:#?}"));
                }
                "vm" | "vector mean" => {
                    let m = get_matrix_input(l);
                    let sum = vector_mean(&m);
                    l.print(&format!("{sum:#?}"));
                }
                "b" | "back" => break,
                _ => l.eprint(
                    "Invalid operation. Valid: 'vs' (vector sum), 'vm' (vector mean), 'b' (back)",
                ),
            }
        }
    }
}
