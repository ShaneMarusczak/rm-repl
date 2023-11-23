#[cfg(test)]
mod rmr_tests {
    use std::collections::HashMap;

    use crate::{evaluate::evaluate, logger::TestLogger, repl::Repl};

    fn get_repl_instance() -> Repl {
        Repl {
            previous_answer: 0.0,
            previous_answer_valid: false,
            variables: HashMap::new(),
        }
    }

    fn get_test_logger() -> TestLogger {
        TestLogger {
            val: String::new(),
            eval: String::new(),
        }
    }

    fn test_initialize() -> (Repl, TestLogger) {
        (get_repl_instance(), get_test_logger())
    }

    #[test]
    fn evaluate_test() {
        let (mut repl, mut test_logger) = test_initialize();
        let line = "(3+2+1)/2";
        evaluate(line, &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "3.00");
    }
}
