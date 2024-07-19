use std::collections::HashMap;

pub(crate) struct Repl {
    pub(crate) previous_answer: f32,
    pub(crate) previous_answer_valid: bool,
    pub(crate) variables: HashMap<char, String>,

    pub(crate) height: usize,
    pub(crate) width: usize,
}

//the Repl object is a user session object
//store results of math problems
//give a session cache, debug flags

impl Repl {
    pub(crate) fn new(previous_answer: f32, previous_answer_valid: bool, width: usize) -> Self {
        Self {
            previous_answer,
            previous_answer_valid,
            variables: HashMap::new(),
            height: width / 2,
            width,
        }
    }

    pub(crate) fn update_dimensions(&mut self, width: usize) {
        self.height = width / 2;
        self.width = width;
    }
    pub(crate) fn set_previous_answer(&mut self, value: &f32) {
        self.previous_answer = *value;
        self.previous_answer_valid = true;
    }
    pub(crate) fn invalidate_prev_answer(&mut self) {
        self.previous_answer = 0.0;
        self.previous_answer_valid = false;
    }
}
