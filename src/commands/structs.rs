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

impl NormalizedPoint {
    pub(crate) const fn new(x: usize, y: usize, y_acc: f32) -> Self {
        Self { x, y, y_acc }
    }
}
