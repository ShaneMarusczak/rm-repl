use std::collections::HashMap;

pub(crate) struct Repl {
    pub(crate) previous_answer: f32,
    pub(crate) previous_answer_valid: bool,
    pub(crate) variables: HashMap<char, String>,

    pub(crate) y_min: f32,
    pub(crate) y_max: f32,

    pub(crate) height: usize,
    pub(crate) width: usize,
}

pub(crate) trait PreviousAnswer {
    fn set_previous_answer(&mut self, value: &f32);
    fn invalidate_prev_answer(&mut self);
}

impl PreviousAnswer for Repl {
    fn set_previous_answer(&mut self, value: &f32) {
        self.previous_answer = *value;
        self.previous_answer_valid = true;
    }
    fn invalidate_prev_answer(&mut self) {
        self.previous_answer = 0.0;
        self.previous_answer_valid = false;
    }
}
