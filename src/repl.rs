pub(crate) struct Repl {
    pub(crate) previous_answer: f32,
    pub(crate) previous_answer_valid: bool,
}

pub(crate) trait PreviousAnswer {
    fn previous_answer(&mut self, value: f32, valid: bool);
}

impl PreviousAnswer for Repl {
    fn previous_answer(&mut self, value: f32, valid: bool) {
        self.previous_answer = value;
        self.previous_answer_valid = valid;
    }
}
