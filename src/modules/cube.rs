use crate::modules::common::*;
use crossterm::{
    cursor::{self, Hide, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::{fmt::Write, time::Duration};

use crate::modules::logger::Logger;

pub(crate) fn cube(l: &mut impl Logger, go: &GraphOptions) {
    let mut stdout = std::io::stdout();
    execute!(stdout, Hide).unwrap();

    let ver_gap = (go.height / 4) as f32;
    let hor_gap = (go.width / 4) as f32;

    let edge_length = f32::min(ver_gap, hor_gap);

    let depth_gap = edge_length;

    let p0 = [hor_gap, ver_gap, depth_gap / 2.0];
    let p1 = [hor_gap, ver_gap + edge_length, depth_gap / 2.0];
    let p2 = [hor_gap + edge_length, ver_gap, depth_gap / 2.0];
    let p3 = [
        hor_gap + edge_length,
        ver_gap + edge_length,
        depth_gap / 2.0,
    ];

    let p4 = [hor_gap, ver_gap, -depth_gap / 2.0];
    let p5 = [hor_gap, ver_gap + edge_length, -depth_gap / 2.0];
    let p6 = [hor_gap + edge_length, ver_gap, -depth_gap / 2.0];
    let p7 = [
        hor_gap + edge_length,
        ver_gap + edge_length,
        -depth_gap / 2.0,
    ];

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
    let frame = make_cube(go, &points, &edges);

    let new_lines = frame
        .chars()
        .filter(|c| c.eq_ignore_ascii_case(&'\n'))
        .count()
        + 1;

    l.print(&frame);

    loop {
        let _ = enable_raw_mode();
        let v = poll(Duration::from_millis(20));
        let _ = disable_raw_mode();

        if v.is_ok_and(|v| v) {
            match read() {
                Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                })) => break,
                _ => continue,
            }
        } else {
            let _ = stdout.execute(cursor::MoveUp(new_lines as u16));

            rotate_points(&mut points, 1.2, 1.5, -1.8);

            let frame = make_cube(go, &points, &edges);
            l.print(&frame);
        }
    }
    execute!(stdout, Show).unwrap();
}

fn rotate_points(
    points: &mut [[f32; 3]],
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

fn make_cube(go: &GraphOptions, p: &[[f32; 3]], edges: &[[usize; 2]]) -> String {
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
