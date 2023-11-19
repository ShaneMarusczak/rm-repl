pub(crate) struct Cell {
    pub(crate) value: bool,
    pub(crate) visited: bool,
}

impl Cell {
    pub(crate) fn new() -> Cell {
        Cell {
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

impl NormalizedPoint {
    pub(crate) fn new(x: usize, y: usize, y_acc: f32) -> NormalizedPoint {
        NormalizedPoint { x, y, y_acc }
    }
}
