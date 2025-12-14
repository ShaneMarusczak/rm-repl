use crate::modules::{
    common::{
        get_braille, make_cell_matrix, CellMatrix, CharMatrix, GraphOptions, NormalizedPoint,
        Point, PointMatrix,
    },
    string_maker::make_graph_string,
};

use rayon::prelude::*;
use rusty_maths::{equation_analyzer::calculator::plot, utilities::abs_f32};

// Sampling factor divisor for determining point density in graphs
const SAMPLING_DIVISOR: f32 = 16.0;
// Maximum allowed difference between actual and default y-range
const Y_RANGE_TOLERANCE: f32 = 50.0;
// Padding added to y-axis bounds
const Y_AXIS_PADDING: f32 = 0.5;

// Tick mark display thresholds based on graph width
const TICK_WIDTH_SMALL: usize = 76;
const TICK_WIDTH_MEDIUM: usize = 151;
const TICK_WIDTH_LARGE: usize = 301;
// Maximum tick marks to display based on graph width
const TICK_MAX_SMALL: usize = 80;
const TICK_MAX_MEDIUM: usize = 160;
const TICK_MAX_LARGE: usize = 300;
const TICK_MAX_XLARGE: usize = 400;

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

    let sampling_factor: f32 = (go.width as f32) / SAMPLING_DIVISOR;

    let x_step: f32 = (x_max - x_min) / ((go.width as f32) * sampling_factor);

    let eqs: Vec<&str> = eq_str.split('|').collect();

    let mut points_collection: PointMatrix = Vec::with_capacity(eqs.len());

    for eq in eqs {
        let rm_points = plot(eq, x_min, x_max, x_step)?;
        let points: Vec<Point> = rm_points
            .into_iter()
            .map(|p| Point::new(p.x, p.y))
            .collect();

        let y_min_actual: f32 = get_y_min(&points);
        let y_max_actual: f32 = get_y_max(&points);

        y_max = if abs_f32(y_max - y_max_actual) < Y_RANGE_TOLERANCE {
            y_max_actual
        } else {
            y_max
        };

        y_min = if abs_f32(y_min - y_min_actual) < Y_RANGE_TOLERANCE {
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

        //Keep NaN from sneaking in
        if points.iter().any(|p| p.x.is_nan() || p.y.is_nan()) {
            let p = points
                .iter()
                .map(|p| {
                    if p.x.is_nan() || p.y.is_nan() {
                        Point::new(0.0, 0.0)
                    } else {
                        Point::new(p.x, p.y)
                    }
                })
                .collect();
            points_collection.push(p);
        } else {
            points_collection.push(points);
        }
    }

    master_y_max += Y_AXIS_PADDING;
    master_y_min -= Y_AXIS_PADDING;

    let mut matrix: CellMatrix = make_cell_matrix(go);

    check_add_tick_marks(&mut matrix, x_min, x_max, master_y_min, master_y_max, go);

    for points in points_collection {
        for np in get_normalized_points(
            go.height,
            master_y_min,
            master_y_max,
            &points,
            sampling_factor,
        )
        .filter(|np| np.y_acc <= master_y_max && np.y_acc >= master_y_min)
        {
            matrix[np.y][np.x].value = true;
        }
    }

    check_add_x_axis(master_y_min, master_y_max, go.height, &mut matrix);

    matrix.reverse();

    check_add_y_axis(x_min, x_max, go.width, &mut matrix);

    let braille_chars: CharMatrix = get_braille(go, &mut matrix);

    Ok(make_graph_string(
        braille_chars,
        x_min,
        x_max,
        master_y_min,
        master_y_max,
    ))
}

fn check_add_tick_marks(
    matrix: &mut CellMatrix,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    go: &GraphOptions,
) {
    let max = if go.width < TICK_WIDTH_SMALL {
        TICK_MAX_SMALL
    } else if go.width < TICK_WIDTH_MEDIUM {
        TICK_MAX_MEDIUM
    } else if go.width < TICK_WIDTH_LARGE {
        TICK_MAX_LARGE
    } else {
        TICK_MAX_XLARGE
    };

    let x_range = x_min.ceil() as isize..=x_max.floor() as isize;

    let y_start = y_min.ceil() as isize;
    let y_end = y_max.floor() as isize;

    let x_scale = (x_max - x_min) / (go.width as f32);
    let y_scale = (y_max - y_min) / (go.height as f32);

    let mut points: Vec<(usize, usize)> = vec![];
    for x in x_range {
        let x_normalized = ((x as f32 - x_min) / x_scale).round() as usize;

        for y in y_start..=y_end {
            let y_normalized = ((y as f32 - y_min) / y_scale).round() as usize;

            if let Some(row) = matrix.get_mut(y_normalized) {
                if row.get_mut(x_normalized).is_some() {
                    points.push((x_normalized, y_normalized));
                }
            }
        }
    }

    if points.len() <= max {
        for (x, y) in points {
            if let Some(row) = matrix.get_mut(y) {
                if let Some(cell) = row.get_mut(x) {
                    cell.value = true;
                }
            }
        }
    }
}

pub(crate) fn get_normalized_points(
    height: usize,
    y_min: f32,
    y_max: f32,
    points: &[Point],
    sampling_factor: f32,
) -> impl Iterator<Item = NormalizedPoint> {
    let y_step = (y_max - y_min) / height as f32;

    let y_values: Vec<f32> = (0..=height)
        .map(|n| y_step.mul_add(n as f32, y_min))
        .collect();

    let inverse_samp_factor = 1.0 / sampling_factor;

    points
        .par_iter()
        .enumerate()
        .map(|(i, point)| {
            let x = ((i as f32) * inverse_samp_factor) as usize;
            let y = binary_search(&y_values, point.y);

            NormalizedPoint {
                x,
                y,
                y_acc: point.y,
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
}

///assumes nums is in ascending order
fn binary_search(nums: &[f32], num: f32) -> usize {
    if nums.is_empty() {
        return 0;
    }
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

fn get_y_min(points: &[Point]) -> f32 {
    points.iter().map(|point| point.y).fold(f32::MAX, f32::min)
}

fn get_y_max(points: &[Point]) -> f32 {
    points.iter().map(|point| point.y).fold(f32::MIN, f32::max)
}

fn x_y_axis_setup(min: f32, max: f32, axis: usize) -> (bool, usize) {
    let axis_in_view: bool = min < 0_f32 && max > 0_f32;

    let axis_ratio: f32 = abs_f32(min) / (max - min);

    let axis_loc: usize = (axis_ratio * axis as f32).round() as usize;
    (axis_in_view, axis_loc)
}
