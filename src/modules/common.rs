/// Represents a point in 2D space for plotting equations.
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Point {
    /// The x-coordinate
    pub x: f32,
    /// The y-coordinate (result of evaluating the equation at x)
    pub y: f32,
}

impl Point {
    /// Creates a new Point with the given coordinates.
    pub(crate) fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

pub(crate) type CellMatrix = Vec<Vec<Cell>>;
pub(crate) type CharMatrix = Vec<Vec<char>>;
pub(crate) type PointMatrix = Vec<Vec<Point>>;

#[derive(Debug)]
pub(crate) struct Cell {
    pub(crate) value: bool,
    pub(crate) visited: bool,
}

impl Cell {
    pub(crate) const fn new() -> Self {
        Self {
            value: false,
            visited: false,
        }
    }
}

pub(crate) struct NormalizedPoint {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) y_acc: f32,
}

pub(crate) struct GraphOptions {
    pub(crate) y_min: f32,
    pub(crate) y_max: f32,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

pub(crate) fn make_cell_matrix(go: &GraphOptions) -> CellMatrix {
    (0..go.height + 1)
        .map(|_| (0..go.width + 1).map(|_| Cell::new()).collect())
        .collect()
}

///converts a matrix of 1s and 0s to a matrix of braille characters with dots at the 1s and blanks at the 0s
///https://en.wikipedia.org/wiki/Braille_Patterns
///
/// ⣿
pub(crate) fn get_braille(go: &GraphOptions, matrix: &mut CellMatrix) -> CharMatrix {
    let row_char_count = go.height / 4;
    let col_char_count = go.width / 2;

    let mut chars: CharMatrix = vec![Vec::with_capacity(col_char_count); row_char_count];

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let cell: &Cell = &matrix[row][col];

            //this cell has already been used in a previous char
            if cell.visited {
                continue;
            }
            let mut braille_char_bits = 0u8;
            let mut shift = 0u8;

            //1-6 braille dots
            for dx in 0..=1 {
                for dy in 0..=2 {
                    if let Some(row_data) = matrix.get_mut(row + dy) {
                        if let Some(cell_data) = row_data.get_mut(col + dx) {
                            //00000000 |= 00000001 (shifted true by 0) -> 00000001
                            //00000001 |= 00000010 (shifted true by 1) -> 00000011
                            //00000011 |= 00000000 (shifted false by 2) -> 00000011
                            //etc..
                            braille_char_bits |= (cell_data.value as u8) << shift;
                            cell_data.visited = true;
                            shift += 1;
                        }
                    }
                }
            }

            //7-8 braille dots
            for dx in 0..=1 {
                let dy = 3;
                if let Some(row_data) = matrix.get_mut(row + dy) {
                    if let Some(cell_data) = row_data.get_mut(col + dx) {
                        braille_char_bits |= (cell_data.value as u8) << shift;
                        cell_data.visited = true;
                        shift += 1;
                    }
                }
            }

            if (row / 4) < chars.len() {
                let braille_char = '⠀' as u32 + braille_char_bits as u32;
                if let Some(v) = std::char::from_u32(braille_char) {
                    chars[row / 4].push(v);
                }
            }
        }
    }
    chars
}

pub(crate) fn draw_line(matrix: &mut CellMatrix, x1: usize, y1: usize, x2: usize, y2: usize) {
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
