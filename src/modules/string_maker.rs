use crate::modules::common::Point;
use std::fmt::Write;

const UPPER_LEFT: &str = "┌";
const UPPER_RIGHT: &str = "┐";
const BOTTOM_LEFT: &str = "└";
const BOTTOM_RIGHT: &str = "┘";
const HORIZONTAL_BAR: &str = "─";
const VERTICAL_BAR: &str = "│";

const UNDERLINE_START: &str = "\u{001b}[4m";
const UNDERLINE_END: &str = "\u{001b}[0m";

pub(crate) fn make_table_string(points: Vec<Point>) -> String {
    let first_row = format!(
        "{}{}{} \n",
        UPPER_LEFT,
        HORIZONTAL_BAR.repeat(13),
        UPPER_RIGHT
    );
    let second_row = format!(
        "{}{} {:<4}{} {:<6}{}{}\n",
        VERTICAL_BAR, UNDERLINE_START, "X", VERTICAL_BAR, "Y", UNDERLINE_END, VERTICAL_BAR
    );

    let middle_rows = points.iter().fold(String::new(), |mut acc, p| {
        writeln!(
            acc,
            "{}{}{:<4} {} {:<6}{}{}",
            VERTICAL_BAR,
            UNDERLINE_START,
            ((p.x * 100.0).round() / 100.0),
            VERTICAL_BAR,
            ((p.y * 100.0).round() / 100.0),
            UNDERLINE_END,
            VERTICAL_BAR
        )
        .unwrap();
        acc
    });

    let last_row = format!(
        "{}{}{} ",
        BOTTOM_LEFT,
        HORIZONTAL_BAR.repeat(13),
        BOTTOM_RIGHT
    );

    first_row + &second_row + &middle_rows + &last_row
}

pub(crate) fn make_graph_string(
    chars: Vec<Vec<char>>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> String {
    let gap = chars.first().map_or(0, |row| row.len());
    let gap_str = HORIZONTAL_BAR.repeat(gap);

    let top_line = format!("{}{}{}{:.2}\n", UPPER_LEFT, gap_str, UPPER_RIGHT, y_max);

    let middle_lines = chars.iter().fold(String::new(), |mut acc, s| {
        writeln!(
            acc,
            "{}{}{}",
            VERTICAL_BAR,
            s.iter().collect::<String>(),
            VERTICAL_BAR
        )
        .unwrap();
        acc
    });

    let bottom_line = format!("{}{}{}{:.2}\n", BOTTOM_LEFT, gap_str, BOTTOM_RIGHT, y_min);

    let x_axis_line = format!("{}{}{}{}", x_min, " ".repeat(gap - 1), x_max, " ".repeat(5));

    top_line + &middle_lines + &bottom_line + &x_axis_line
}

pub(crate) fn make_curve_string(
    chars: Vec<Vec<char>>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> String {
    let gap = chars.first().map_or(0, |row| row.len());
    let gap_str = HORIZONTAL_BAR.repeat(gap);

    let top_line = format!("{}{}{}{:.2}\n", UPPER_LEFT, gap_str, UPPER_RIGHT, y_min);

    let middle_lines = chars.iter().fold(String::new(), |mut acc, s| {
        writeln!(
            acc,
            "{}{}{}",
            VERTICAL_BAR,
            s.iter().collect::<String>(),
            VERTICAL_BAR
        )
        .unwrap();
        acc
    });

    let bottom_line = format!("{}{}{}{:.2}\n", BOTTOM_LEFT, gap_str, BOTTOM_RIGHT, y_max);

    let x_axis_line = format!(
        "\n{}{}{}{}\n",
        x_min,
        " ".repeat(gap - 1),
        x_max,
        " ".repeat(5)
    );

    x_axis_line + &top_line + &middle_lines + &bottom_line
}
