use crate::string_maker::make_graph_string;
use crate::structs::{Cell, GraphOptions, NormalizedPoint};
use rusty_maths::{
    equation_analyzer::{calculator::plot, eq_data_builder::Point},
    utilities::abs_f32,
};

use std::sync::Arc;
use std::thread;

type CellMatrix = Vec<Vec<Cell>>;
type PointMatrix = Vec<Vec<Point>>;
type CharMatrix = Vec<Vec<char>>;

pub(crate) fn graph(
    eq_str: &str,
    x_min: f32,
    x_max: f32,
    go: &GraphOptions,
) -> Result<String, String> {
    let mut y_min: f32 = go.y_min;
    let mut y_max: f32 = go.y_max;

    let mut master_y_min: f32 = f32::MAX;
    let mut master_y_max: f32 = f32::MIN;

    //still fiddling trying to find the correct value
    let sampling_factor: f32 = (go.width / 16) as f32;

    let x_step: f32 = (x_max - x_min) / ((go.width as f32) * sampling_factor);

    let eqs: Vec<&str> = eq_str.split('|').collect();

    let mut matrix: CellMatrix = make_cell_matrix(go.height + 1, go.width + 1);

    let mut points_collection: PointMatrix = Vec::with_capacity(eqs.len());

    for eq in eqs {
        let points: Vec<Point> = plot(eq, x_min, x_max, x_step)?;

        let y_min_actual: f32 = get_y_min(&points);
        let y_max_actual: f32 = get_y_max(&points);

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

        if y_min < master_y_min {
            master_y_min = y_min;
        }
        if y_max > master_y_max {
            master_y_max = y_max;
        }
        points_collection.push(points);
    }

    master_y_max += 0.5;
    master_y_min -= 0.5;

    if master_y_max - master_y_min < 11.0 && x_max - x_min < 11.0 {
        add_tick_marks(
            &mut matrix,
            x_min,
            x_max,
            master_y_min,
            master_y_max,
            go.height,
            go.width,
        );
    }

    for points in points_collection {
        for np in get_normalized_points(
            go.height,
            master_y_min,
            master_y_max,
            &points,
            sampling_factor,
        )
        .iter()
        .filter(|np| np.y_acc < master_y_max && np.y_acc > master_y_min)
        {
            matrix[np.y][np.x].value = true;
        }
    }

    check_add_x_axis(master_y_min, master_y_max, go.height, &mut matrix);

    matrix.reverse();

    check_add_y_axis(x_min, x_max, go.width, &mut matrix);

    let braille_chars: CharMatrix = get_braille(go.height, go.width, &mut matrix);

    Ok(make_graph_string(
        braille_chars,
        x_min,
        x_max,
        master_y_min,
        master_y_max,
    ))
}

fn add_tick_marks(
    matrix: &mut CellMatrix,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    height: usize,
    width: usize,
) {
    let x_range = x_min.ceil() as isize..=x_max.floor() as isize;
    let y_range = y_min.ceil() as isize..=y_max.floor() as isize;

    let x_scale = (x_max - x_min) / (width as f32);
    let y_scale = (y_max - y_min) / (height as f32);

    for x in x_range {
        let x_normalized = ((x as f32 - x_min) / x_scale).round() as usize;

        for y in y_range.clone() {
            let y_normalized = ((y as f32 - y_min) / y_scale).round() as usize;

            if let Some(row) = matrix.get_mut(y_normalized) {
                if let Some(cell) = row.get_mut(x_normalized) {
                    cell.value = true;
                }
            }
        }
    }
}

fn get_normalized_points(
    height: usize,
    y_min: f32,
    y_max: f32,
    points: &Vec<Point>,
    sampling_factor: f32,
) -> Vec<NormalizedPoint> {
    let y_step = (y_max - y_min) / height as f32;

    let y_values: Arc<Vec<f32>> = Arc::new(
        (0..=height)
            .map(|n| y_step.mul_add(n as f32, y_min))
            .collect(),
    );

    let num_threads: usize = num_cpus::get();
    let chunk_size: usize = (points.len() / num_threads) + 1;

    let mut threads = Vec::with_capacity(num_threads);

    let points_chunks: PointMatrix = points.chunks(chunk_size).map(|p| p.into()).collect();

    let inverse_samp_factor = 1.0 / sampling_factor;

    for (c, chunk) in points_chunks.into_iter().enumerate() {
        let y_values: Arc<Vec<f32>> = Arc::clone(&y_values);

        threads.push(thread::spawn(move || {
            let mut thread_results: Vec<NormalizedPoint> = Vec::with_capacity(chunk_size);
            let chunk_offset = c * chunk_size;

            for (i, point) in chunk.iter().enumerate() {
                let x = (((i + chunk_offset) as f32) * inverse_samp_factor) as usize;

                let y = binary_search(Arc::clone(&y_values), point.y);

                thread_results.push(NormalizedPoint {
                    x,
                    y,
                    y_acc: point.y,
                });
            }
            thread_results
        }));
    }

    let mut normalized_points: Vec<NormalizedPoint> = Vec::with_capacity(points.len());

    for thread in threads {
        normalized_points.append(&mut thread.join().unwrap());
    }
    normalized_points
}

///assumes nums is in ascending order
fn binary_search(nums: Arc<Vec<f32>>, num: f32) -> usize {
    if nums[0] >= num {
        return 0;
    }
    if nums[nums.len() - 1] <= num {
        return nums.len() - 1;
    }

    let mut start = 0;
    let mut end = nums.len();

    while start <= end {
        let mut mid = start + (end - start) / 2;

        // usize math pushes mid to zero when trying to compare index 0 and 1
        if mid == 0 {
            mid = 1;
        }

        let mid_minus_one = mid - 1;

        if num >= nums[mid_minus_one] && num <= nums[mid] {
            let check = (num - nums[mid_minus_one]).abs() < (num - nums[mid]).abs();
            return if check { mid_minus_one } else { mid };
        } else if nums[mid] < num {
            start = mid + 1;
        } else {
            end = mid_minus_one;
        }
    }
    unreachable!()
}

///converts a matrix of 1s and 0s to a matrix of braille characters with dots at the 1s and blanks at the 0s
///https://en.wikipedia.org/wiki/Braille_Patterns
///
/// ⣿
fn get_braille(height: usize, width: usize, matrix: &mut CellMatrix) -> CharMatrix {
    let mut chars: CharMatrix = Vec::with_capacity(height / 4);

    for _ in 0..(height / 4) {
        chars.push(Vec::with_capacity(width / 2));
    }

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let cell: &Cell = &matrix[row][col];

            //this cell has already been used in a previous char
            if cell.visited {
                continue;
            }

            //represents a single braille char
            let mut char: Vec<u8> = Vec::with_capacity(8);

            //1-6 braille dots
            for dx in 0..=1 {
                for dy in 0..=2 {
                    if let Some(row_data) = matrix.get_mut(row + dy) {
                        if let Some(cell_data) = row_data.get_mut(col + dx) {
                            char.push(cell_data.value as u8);
                            cell_data.visited = true;
                        }
                    }
                }
            }

            //7-8 braille dots
            for dx in 0..=1 {
                let dy = 3;
                if let Some(row_data) = matrix.get_mut(row + dy) {
                    if let Some(cell_data) = row_data.get_mut(col + dx) {
                        char.push(cell_data.value as u8);
                        cell_data.visited = true;
                    }
                }
            }

            //each braille char contains 4 rows
            if (row / 4) < chars.len() {
                //converts array of 0 and 1 to braille char
                let binary_string: String =
                    char.iter().rev().map(|b| b.to_string()).collect::<String>();

                let decimal_number: u8 = u8::from_str_radix(&binary_string, 2).unwrap();

                let code_point: u32 =
                    u32::from_str_radix(&format!("28{:02x}", decimal_number), 16).unwrap();

                let character: char = char::from_u32(code_point).unwrap();

                chars[row / 4].push(character);
            }
        }
    }
    chars
}

fn check_add_x_axis(y_min: f32, y_max: f32, height: usize, matrix: &mut CellMatrix) {
    let (x_axis_in_view, x_axis_row): (bool, usize) = x_y_axis_setup(y_min, y_max, height);

    if x_axis_in_view {
        if let Some(row) = matrix.get_mut(x_axis_row) {
            for c in &mut *row {
                c.value = true;
            }
        }
    }
}

fn check_add_y_axis(x_min: f32, x_max: f32, width: usize, matrix: &mut CellMatrix) {
    let (y_axis_in_view, y_axis_col): (bool, usize) = x_y_axis_setup(x_min, x_max, width);

    if y_axis_in_view {
        for row in matrix {
            row[y_axis_col].value = true;
        }
    }
}

pub(crate) fn get_y_min(points: &Vec<Point>) -> f32 {
    let mut y_min: f32 = f32::MAX;
    for point in points {
        if point.y < y_min {
            y_min = point.y;
        }
    }
    y_min
}

pub(crate) fn get_y_max(points: &Vec<Point>) -> f32 {
    let mut y_max: f32 = f32::MIN;
    for point in points {
        if point.y > y_max {
            y_max = point.y;
        }
    }
    y_max
}

fn make_cell_matrix(vec_count: usize, vec_length: usize) -> CellMatrix {
    (0..vec_count)
        .map(|_| (0..vec_length).map(|_| Cell::new()).collect())
        .collect()
}

fn x_y_axis_setup(min: f32, max: f32, axis: usize) -> (bool, usize) {
    let axis_in_view: bool = min < 0_f32 && max > 0_f32;

    let axis_ratio: f32 = abs_f32(min) / (max - min);

    let axis_loc: usize = (axis_ratio * axis as f32).round() as usize;
    (axis_in_view, axis_loc)
}
