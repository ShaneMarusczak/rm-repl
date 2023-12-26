use crate::modules::common::*;
use crossterm::{
    cursor::{self, DisableBlinking, EnableBlinking},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::{fmt::Write, time::Duration};

use crate::modules::logger::Logger;

pub(crate) fn cube(l: &mut impl Logger, go: &GraphOptions) {
    let mut stdout = std::io::stdout();

    let ver_gap = (go.height / 4) as f32;
    let hor_gap = (go.width / 4) as f32;
    let depth = (go.height / 4) as f32;

    let p0 = [hor_gap, ver_gap, depth / 2.0];
    let p1 = [hor_gap, 2. * ver_gap, depth / 2.0];
    let p2 = [2. * hor_gap, ver_gap, depth / 2.0];
    let p3 = [2. * hor_gap, 2. * ver_gap, depth / 2.0];

    let p4 = [hor_gap, ver_gap, -depth / 2.0];
    let p5 = [hor_gap, 2. * ver_gap, -depth / 2.0];
    let p6 = [2. * hor_gap, ver_gap, -depth / 2.0];
    let p7 = [2. * hor_gap, 2. * ver_gap, -depth / 2.0];

    let origin = [(p0[0] + p3[0]) / 2.0, (p0[1] + p3[1]) / 2.0, 0.0];

    let edges: Vec<[usize; 2]> = vec![
        [0, 1],
        [1, 3],
        [3, 2],
        [2, 0],
        [4, 5],
        [5, 7],
        [7, 6],
        [6, 4],
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7],
    ];
    let mut points = vec![p0, p1, p2, p3, p4, p5, p6, p7, origin];
    let cubee = make_cube(go, points.clone(), &edges);
    execute!(stdout, DisableBlinking).unwrap();
    let new_lines = cubee
        .chars()
        .filter(|c| c.eq_ignore_ascii_case(&'\n'))
        .count()
        + 1;

    l.print(&cubee);

    loop {
        enable_raw_mode().unwrap();
        let v = poll(Duration::from_millis(100)).unwrap();
        disable_raw_mode().unwrap();
        if v {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => break,
                _ => continue,
            }
        } else {
            stdout
                .execute(cursor::MoveUp(new_lines.try_into().unwrap()))
                .unwrap();
            rotate_points(&mut points, 10., 11., 12.);
            let cubee = make_cube(go, points.clone(), &edges);
            l.print(&cubee);
        }
    }
    execute!(stdout, EnableBlinking).unwrap();
}

fn rotate_points(
    points: &mut Vec<[f32; 3]>,
    angle_degrees_x: f32,
    angle_degrees_y: f32,
    angle_degrees_z: f32,
) {
    let angle_radians_x = angle_degrees_x.to_radians();
    let angle_radians_y = angle_degrees_y.to_radians();
    let angle_radians_z = angle_degrees_z.to_radians();

    let (sin_x, cos_x) = angle_radians_x.sin_cos();
    let (sin_y, cos_y) = angle_radians_y.sin_cos();
    let (sin_z, cos_z) = angle_radians_z.sin_cos();

    let p0 = points[points.len() - 1];

    for i in 0..points.len() - 1 {
        let mut x = points[i][0] - p0[0];
        let y = points[i][1] - p0[1];
        let z = points[i][2] - p0[2];

        let y_on_x = y * cos_x - z * sin_x;
        let z_on_x = y * sin_x + z * cos_x;

        let z_on_y = z_on_x * cos_y - x * sin_y;
        x = x * cos_y + z_on_x * sin_y;

        points[i][0] = x * cos_z - y_on_x * sin_z;
        points[i][1] = x * sin_z + y_on_x * cos_z;
        points[i][2] = z_on_y;

        points[i][0] += p0[0];
        points[i][1] += p0[1];
        points[i][2] += p0[2];
    }
}

fn make_cube(go: &GraphOptions, p: Vec<[f32; 3]>, edges: &[[usize; 2]]) -> String {
    let mut matrix: CellMatrix = make_cell_matrix(go);

    for edge in edges {
        let p_1 = p[edge[0]];
        let p_2 = p[edge[1]];
        draw_line(
            &mut matrix,
            p_1[0] as usize,
            p_1[1] as usize,
            p_2[0] as usize,
            p_2[1] as usize,
        );
    }

    let braille_chars: CharMatrix = get_braille(go, &mut matrix);

    let lines = braille_chars.iter().fold(String::new(), |mut acc, s| {
        writeln!(acc, "{}", s.iter().collect::<String>(),).unwrap();
        acc
    });
    lines
}

fn draw_line(matrix: &mut CellMatrix, x1: usize, y1: usize, x2: usize, y2: usize) {
    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;
    let steep = dy.abs() > dx.abs();

    let (x1, y1, x2, y2) = if steep {
        (y1, x1, y2, x2)
    } else {
        (x1, y1, x2, y2)
    };

    let (x1, x2, y1, y2) = if x1 > x2 {
        (x2, x1, y2, y1)
    } else {
        (x1, x2, y1, y2)
    };

    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y1 as isize;

    for x in x1..=x2 {
        if steep {
            if let Some(col) = matrix.get_mut(x) {
                if let Some(cell) = col.get_mut(y as usize) {
                    cell.value = true;
                }
            }
        } else if let Some(row) = matrix.get_mut(y as usize) {
            if let Some(cell) = row.get_mut(x) {
                cell.value = true;
            }
        }
        error2 += derror2;

        if error2 > dx {
            y += if y2 > y1 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}
