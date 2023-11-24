#[cfg(test)]
mod rmr_tests {
    use std::collections::HashMap;

    use crate::{evaluate::evaluate, logger::TestLogger, repl::Repl};

    fn get_repl() -> Repl {
        Repl {
            previous_answer: 0.0,
            previous_answer_valid: false,
            variables: HashMap::new(),
        }
    }

    fn get_test_logger() -> TestLogger {
        TestLogger {
            val: String::new(),
            error_val: String::new(),
        }
    }

    fn get_repl_and_logger() -> (Repl, TestLogger) {
        (get_repl(), get_test_logger())
    }

    #[test]
    fn evaluate_test() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let line = "(3+2+1)/2";

        //When
        evaluate(line, &mut repl, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "3.00");
        assert_eq!(test_logger.error_val, "");
        assert!(repl.previous_answer_valid);
        assert_eq!(repl.previous_answer, 3f32);
    }

    #[test]
    fn evaluate_error_test() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let line = "(3+2+1)_2";

        //When
        evaluate(line, &mut repl, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "");
        assert_eq!(test_logger.error_val, "Invalid input at character 8");
        assert!(!repl.previous_answer_valid);
        assert_eq!(repl.previous_answer, 0.0);
    }
}
