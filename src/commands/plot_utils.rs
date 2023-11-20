use rusty_maths::{
    equation_analyzer::{calculator::plot, eq_data_builder::Point},
    utilities::abs_f32,
};

use super::structs::{Cell, NormalizedPoint};

pub(crate) fn graph(
    eq: &str,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    width: usize,
    height: usize,
) -> Result<String, String> {
    let mut y_min = y_min;
    let mut y_max = y_max;
    let multiplier = (width / 8) as f32;

    let x_step = (x_max - x_min) / ((width as f32) * multiplier);

    let points = plot(eq, x_min, x_max, x_step)?;
    let y_min_actual = get_y_min(&points);
    let y_max_actual = get_y_max(&points);

    y_max = if abs_f32(y_max - y_max_actual) < 50_f32 {
        y_max_actual
    } else {
        y_max
    };

    y_min = if abs_f32(y_min - y_min_actual) < 50_f32 {
        y_min_actual
    } else {
        y_min
    };

    y_max += 0.5;
    y_min -= 0.5;

    let mut matrix = make_matrix(height + 1, width + 1);

    for np in get_normalized_points(height, y_min, y_max, &points, multiplier)
        .iter()
        .filter(|np| np.y_acc < y_max && np.y_acc > y_min)
    {
        matrix[np.y][np.x].value = true;
    }

    check_add_x_axis(y_min, y_max, height, &mut matrix);

    matrix.reverse();

    check_add_y_axis(x_min, x_max, width, &mut matrix);

    let braille_chars = get_braille(height, width, &mut matrix);

    Ok(get_graph_string(braille_chars, x_min, x_max, y_min, y_max))
}

///converts points to location on screen
///-10..10 and -100..100 both need to fit on the same amount of screen real estate
fn get_normalized_points(
    height: usize,
    y_min: f32,
    y_max: f32,
    points: &[Point],
    multiplier: f32,
) -> Vec<NormalizedPoint> {
    use std::sync::Arc;
    use std::thread;

    let y_values: Arc<Vec<f32>> = Arc::new(
        (0..=height)
            .map(|n| ((y_max - y_min) / height as f32).mul_add(n as f32, y_min))
            .collect(),
    );

    let num_threads = num_cpus::get();
    let chunk_size = (points.len() / num_threads) + 1;

    let mut threads = Vec::with_capacity(num_threads);

    let points_chunks: Vec<Vec<Point>> = points.chunks(chunk_size).map(|p| p.into()).collect();

    for (c, chunk) in points_chunks.into_iter().enumerate() {
        let y_values = Arc::clone(&y_values);
        threads.push(thread::spawn(move || {
            let mut thread_results = vec![];
            for (i, point) in chunk.iter().enumerate() {
                let x = (i + (c * chunk.len())) / multiplier as usize;

                let mut min_dif = f32::MAX;
                let mut y = 0;
                for (i, p) in y_values.iter().enumerate() {
                    let dif = abs_f32(point.y - p);
                    if dif < min_dif {
                        min_dif = dif;
                        y = i;
                    }
                }

                let n = NormalizedPoint::new(x, y, point.y);
                thread_results.push(n);
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

///adds frame and x y mins and maxes
fn get_graph_string(
    chars: Vec<Vec<char>>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> String {
    let gap = chars.first().map_or(0, |row| row.len());
    let mut s = format!("{}\n", "-".repeat(gap + 1));

    for (i, row) in chars.iter().enumerate() {
        s.push('|');
        for cell in row {
            s.push(*cell);
        }
        s.push('|');
        if i == 0 {
            s += &format!("{:.2}", y_max);
        } else if i == chars.len() - 1 {
            s += &format!("{:.2}", y_min);
        }
        s.push('\n');
    }
    s += &format!("{}\n", "-".repeat(gap + 1));
    s += &format!("{}{}{}{}", x_min, " ".repeat(gap - 2), x_max, " ".repeat(5));

    s
}

///converts a matrix of 1s and 0s to a matrix of braille with dots at the 1s and blanks at the 0s
///https://en.wikipedia.org/wiki/Braille_Patterns
fn get_braille(height: usize, width: usize, matrix: &mut Vec<Vec<Cell>>) -> Vec<Vec<char>> {
    let mut chars = Vec::with_capacity(height / 4);
    for _ in 0..(height / 4) {
        chars.push(Vec::with_capacity(width / 2));
    }
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let cell = &matrix[row][col];
            //if this cell has already been accounted for in a previous char, braille chars are 2 columns
            if cell.visited {
                continue;
            }
            //represents a single braille char
            let mut char = Vec::with_capacity(8);
            //1-6 braille dots
            for dx in 0..=1 {
                for dy in 0..=2 {
                    if let Some(row_data) = matrix.get(row + dy) {
                        if let Some(cell_data) = row_data.get(col + dx) {
                            char.push(cell_data.value as u8);
                            matrix[row + dy][col + dx].visited = true;
                        }
                    }
                }
            }
            //7-8 braille dots
            for dx in 0..=1 {
                let dy = 3;
                if let Some(row_data) = matrix.get(row + dy) {
                    if let Some(cell_data) = row_data.get(col + dx) {
                        char.push(cell_data.value as u8);
                        matrix[row + dy][col + dx].visited = true;
                    }
                }
            }
            if (row / 4) < chars.len() {
                char.reverse();

                //converts array of 0 and 1 to braille char
                let binary_string = char.iter().map(|b| b.to_string()).collect::<String>();
                let decimal_number = u8::from_str_radix(&binary_string, 2).unwrap();
                let code_point =
                    u32::from_str_radix(&format!("28{:02x}", decimal_number), 16).unwrap();
                let character = char::from_u32(code_point).unwrap();

                //each braille char is actually 4 rows
                chars[row / 4].push(character);
            }
        }
    }
    chars
}

fn check_add_x_axis(y_min: f32, y_max: f32, height: usize, matrix: &mut [Vec<Cell>]) {
    let (x_axis_in_view, x_axis_row) = x_y_axis_setup(y_min, y_max, height);

    if x_axis_in_view {
        if let Some(row) = matrix.get_mut(x_axis_row) {
            for c in &mut *row {
                c.value = true;
            }
        }
    }
}

fn check_add_y_axis(x_min: f32, x_max: f32, width: usize, matrix: &mut [Vec<Cell>]) {
    let (y_axis_in_view, y_axis_col) = x_y_axis_setup(x_min, x_max, width);
    if y_axis_in_view {
        for row in matrix {
            row[y_axis_col].value = true;
        }
    }
}

fn get_y_min(points: &[Point]) -> f32 {
    let mut y_min = f32::MAX;
    for point in points {
        if point.y < y_min {
            y_min = point.y;
        }
    }
    y_min
}

fn get_y_max(points: &[Point]) -> f32 {
    let mut y_max = f32::MIN;
    for point in points {
        if point.y > y_max {
            y_max = point.y;
        }
    }
    y_max
}

fn make_matrix(vec_count: usize, vec_length: usize) -> Vec<Vec<Cell>> {
    (0..vec_count)
        .map(|_| (0..vec_length).map(|_| Cell::new()).collect())
        .collect()
}

fn x_y_axis_setup(min: f32, max: f32, axis: usize) -> (bool, usize) {
    let axis_in_view = min < 0_f32 && max > 0_f32;

    let axis_ratio: f32 = abs_f32(min) / (max - min);

    let axis_loc = (axis_ratio * axis as f32).round() as usize;
    (axis_in_view, axis_loc)
}
