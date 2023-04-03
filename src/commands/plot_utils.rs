use rusty_maths::utilities::abs_f32;

use crate::inputs::{get_numerical_input, get_textual_input};

pub(crate) fn get_p_inputs() -> (String, f32, f32) {
    let eq = get_textual_input("equation: ");

    let x_min = get_numerical_input("x min: ");

    let x_max = get_numerical_input("x max: ");

    (eq, x_min, x_max)
}

pub(crate) fn get_normalized_points(
    height: usize,
    y_min: f32,
    y_max: f32,
    points: &[(f32, f32)],
    multiplier: f32,
) -> Vec<(usize, (usize, f32))> {
    use std::sync::Arc;
    use std::thread;

    let y_values: Arc<Vec<f32>> = Arc::new(
        (0..=height)
            .map(|n| y_min + (((y_max - y_min) / height as f32) * n as f32))
            .collect(),
    );

    let num_threads = num_cpus::get();
    let chunk_size = (points.len() / num_threads) + 1;

    let mut threads = Vec::with_capacity(num_threads);

    let points_chunks: Vec<Vec<_>> = points.chunks(chunk_size).map(|s| s.into()).collect();

    for (c, chunk) in points_chunks.into_iter().enumerate() {
        let y_values = Arc::clone(&y_values);
        threads.push(thread::spawn(move || {
            let mut thread_results = vec![];
            for (i, point) in chunk.iter().enumerate() {
                let x = (i + (c * chunk.len())) / multiplier as usize;
                let y = get_norm_y_value(&y_values, *point);
                thread_results.push((x, y));
            }
            thread_results
        }));
    }

    let mut normalized_points = Vec::with_capacity(points.len());

    for thread in threads {
        normalized_points.append(&mut thread.join().unwrap());
    }
    normalized_points
}

pub(crate) fn get_graph_string(
    chars: Vec<Vec<char>>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> String {
    let gap = chars.first().unwrap().len();
    let mut s = format!("{}\n", "-".repeat(gap + 1));

    for (i, row) in chars.iter().enumerate() {
        s.push('|');
        for cell in row {
            s.push(*cell);
        }
        s.push('|');
        if i == 0 {
            let y_max = (y_max * 1000.0).round() / 1000.0;
            s = s + &format!("{}", y_max);
        } else if i == chars.len() - 1 {
            let y_min = (y_min * 1000.0).round() / 1000.0;
            s = s + &format!("{}", y_min);
        }
        s.push('\n');
    }
    s += &format!("{}\n", "-".repeat(gap + 1));
    s += &format!("{}{}{}", x_min, " ".repeat(gap - 2), x_max);

    s
}

pub(crate) fn get_braille(
    height: usize,
    width: usize,
    matrix: &mut Vec<Vec<(u8, bool)>>,
) -> Vec<Vec<char>> {
    let mut chars = Vec::with_capacity(height / 4);
    for _ in 0..(height / 4) {
        chars.push(Vec::with_capacity(width / 2));
    }
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let cell = matrix[row][col];
            if cell.1 {
                continue;
            }
            let mut char = vec![];
            for dx in 0..=1 {
                for dy in 0..=2 {
                    if row + dy < matrix.len() && col + dx < matrix[row].len() {
                        let val = matrix[row + dy][col + dx];
                        char.push(val.0);
                        matrix[row + dy][col + dx].1 = true;
                    }
                }
            }
            for dx in 0..=1 {
                let dy = 3;
                if row + dy < matrix.len() && col + dx < matrix[row].len() {
                    let val = matrix[row + dy][col + dx];
                    char.push(val.0);
                    matrix[row + dy][col + dx].1 = true;
                }
            }
            if (row / 4) < chars.len() {
                char.reverse();

                let binary_string = char.iter().map(|b| b.to_string()).collect::<String>();
                let decimal_number = u8::from_str_radix(&binary_string, 2).unwrap();
                let code_point =
                    u32::from_str_radix(&format!("28{:02x}", decimal_number), 16).unwrap();
                let character = std::char::from_u32(code_point).unwrap();

                chars[row / 4].push(character);
            }
        }
    }
    chars
}

pub(crate) fn check_add_x_axis(
    y_min: f32,
    y_max: f32,
    height: usize,
    matrix: &mut [Vec<(u8, bool)>],
) {
    let (x_axis_in_view, x_axis_row) = x_y_axis_setup(y_min, y_max, height);

    if x_axis_in_view {
        if let Some(row) = matrix.get_mut(x_axis_row) {
            for c in row.iter_mut() {
                c.0 = 1;
            }
        }
    }
}

pub(crate) fn check_add_y_axis(
    x_min: f32,
    x_max: f32,
    width: usize,
    matrix: &mut [Vec<(u8, bool)>],
) {
    let (y_axis_in_view, y_axis_col) = x_y_axis_setup(x_min, x_max, width);
    if y_axis_in_view {
        for row in matrix {
            row[y_axis_col].0 = 1;
        }
    }
}

pub(crate) fn get_y_min_max(points: &[(f32, f32)]) -> (f32, f32) {
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;

    for point in points {
        if point.1 < y_min {
            y_min = point.1;
        } else if point.1 > y_max {
            y_max = point.1;
        }
    }
    (y_min, y_max)
}

pub(crate) fn get_norm_y_value(normalized_y_values: &[f32], point: (f32, f32)) -> (usize, f32) {
    let mut min_dif = f32::MAX;

    let mut rv = 0;
    for (i, p) in normalized_y_values.iter().enumerate() {
        let dif = abs_f32(point.1 - p);
        if dif < min_dif {
            min_dif = dif;
            rv = i;
        }
    }
    (rv, point.1)
}

pub(crate) fn make_matrix(vec_count: usize, vec_length: usize) -> Vec<Vec<(u8, bool)>> {
    (0..vec_count)
        .map(|_| (0..vec_length).map(|_| (0, false)).collect())
        .collect()
}

pub(crate) fn x_y_axis_setup(min: f32, max: f32, axis: usize) -> (bool, usize) {
    let axis_in_view = min < 0_f32 && max > 0_f32;

    let axis_ratio: f32 = abs_f32(min) / (max - min);

    let axis_loc = (axis_ratio * axis as f32).round() as usize;
    (axis_in_view, axis_loc)
}
