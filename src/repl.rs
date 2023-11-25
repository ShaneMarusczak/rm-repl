use std::collections::HashMap;

pub(crate) struct Repl {
    pub(crate) previous_answer: f32,
    pub(crate) previous_answer_valid: bool,
    pub(crate) variables: HashMap<char, String>,
}

pub(crate) trait PreviousAnswer {
    fn previous_answer(&mut self, value: &f32, valid: bool);
    fn invalidate_prev_answer(&mut self);
}

impl PreviousAnswer for Repl {
    fn previous_answer(&mut self, value: &f32, valid: bool) {
        self.previous_answer = *value;
        self.previous_answer_valid = valid;
    }
    fn invalidate_prev_answer(&mut self) {
        self.previous_answer = 0.0;
        self.previous_answer_valid = false;
    }
}
