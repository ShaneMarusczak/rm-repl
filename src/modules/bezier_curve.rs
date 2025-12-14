use super::{
    common::{get_braille, make_cell_matrix, CellMatrix, CharMatrix, GraphOptions, Point},
    logger::Logger,
    string_maker::make_curve_string,
};

const STEP_SIZE: f32 = 0.001;

// Helper function to add control point markers with offset
fn add_control_point_markers(
    points: &mut Vec<Point>,
    control_points: &[Point],
    width: f32,
    height: f32,
) {
    let offsets = [-1.0, 1.0];
    for control_point in control_points {
        for &ox in &offsets {
            for &oy in &offsets {
                let new_x = (control_point.x + ox).clamp(0.0, width - 1.0);
                let new_y = (control_point.y + oy).clamp(0.0, height - 1.0);
                points.push(Point::new(new_x, new_y));
            }
        }
        points.push(control_point.clone());
    }
}

// Helper function to render curve points to matrix
fn render_curve(
    points: Vec<Point>,
    go: &GraphOptions,
    l: &mut impl Logger,
) {
    let mut matrix: CellMatrix = make_cell_matrix(go);

    for p in points {
        if let Some(col) = matrix.get_mut(p.y as usize) {
            if let Some(cell) = col.get_mut(p.x as usize) {
                cell.value = true;
            }
        }
    }

    let braille_chars: CharMatrix = get_braille(go, &mut matrix);
    l.print(&make_curve_string(
        braille_chars,
        0.0,
        go.width as f32,
        0.0,
        go.height as f32,
    ));
}

pub(crate) fn quadratic_bezier(
    p1: Point,
    p2: Point,
    p3: Point,
    go: &GraphOptions,
    l: &mut impl Logger,
) {
    let width = go.width as f32;
    let height = go.height as f32;

    let mut points = vec![p1.clone()];
    add_control_point_markers(&mut points, &[p2.clone()], width, height);
    points.push(p3.clone());

    let mut n = 0.0;
    while n <= 1.01 {
        let a_x = interpolate(p1.x, p2.x, n);
        let a_y = interpolate(p1.y, p2.y, n);
        let b_x = interpolate(p2.x, p3.x, n);
        let b_y = interpolate(p2.y, p3.y, n);
        let z_x = interpolate(a_x, b_x, n);
        let z_y = interpolate(a_y, b_y, n);

        points.push(Point::new(z_x, z_y));
        n += STEP_SIZE;
    }

    render_curve(points, go, l);
}

pub(crate) fn cubic_bezier(
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    go: &GraphOptions,
    l: &mut impl Logger,
) {
    let width = go.width as f32;
    let height = go.height as f32;

    let mut points = vec![p1.clone()];
    add_control_point_markers(&mut points, &[p2.clone(), p3.clone()], width, height);
    points.push(p4.clone());

    let mut n = 0.0;
    while n <= 1.01 {
        let a_x = interpolate(p1.x, p2.x, n);
        let a_y = interpolate(p1.y, p2.y, n);

        let b_x = interpolate(p2.x, p3.x, n);
        let b_y = interpolate(p2.y, p3.y, n);

        let c_x = interpolate(p3.x, p4.x, n);
        let c_y = interpolate(p3.y, p4.y, n);

        let d_x = interpolate(a_x, b_x, n);
        let d_y = interpolate(a_y, b_y, n);

        let e_x = interpolate(b_x, c_x, n);
        let e_y = interpolate(b_y, c_y, n);

        let z_x = interpolate(d_x, e_x, n);
        let z_y = interpolate(d_y, e_y, n);
        points.push(Point::new(z_x, z_y));

        n += STEP_SIZE;
    }

    render_curve(points, go, l);
}

fn interpolate(start: f32, end: f32, t: f32) -> f32 {
    start + t * (end - start)
}
