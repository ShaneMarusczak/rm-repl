#[cfg(test)]
mod rmr_tests {
    use std::collections::HashMap;

    use crate::{
        evaluate::{evaluate, simple_evaluate},
        logger::TestLogger,
        repl::Repl,
        run::as_cli_tool,
        variables::{handle_var, is_variable},
    };

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

    fn is_graph_string(g: &str) -> bool {
        //The only place in the program these three chars are used in in the creation of a valid graph
        let empty_braille_char = '⠀';
        let upper_left = '┌';
        let upper_right = '┐';

        g.contains(empty_braille_char) && g.contains(upper_left) && g.contains(upper_right)
    }

    #[test]
    fn evaluate_test() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let line = "max(2,3)-min(10,11)";

        //When
        evaluate(line, &mut repl, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "-7.00");
        assert!(test_logger.error_val.is_empty());
        assert!(repl.previous_answer_valid);
        assert_eq!(repl.previous_answer, -7f32);
    }

    #[test]
    fn evaluate_error_test() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let line = "(3+2+1)_2";

        //When
        evaluate(line, &mut repl, &mut test_logger);

        //Then
        assert!(test_logger.val.is_empty());
        assert_eq!(test_logger.error_val, "Invalid input at character 8");
        assert!(!repl.previous_answer_valid);
        assert_eq!(repl.previous_answer, 0.0);
    }

    #[test]
    fn simple_evaluate_test() {
        //Given
        let mut test_logger = get_test_logger();
        let line = "(3+2+1)/2";

        //When
        simple_evaluate(line, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "3.00");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn simple_evaluate_error_test() {
        //Given
        let mut test_logger = get_test_logger();
        let line = "(3+2+1)_2";

        //When
        simple_evaluate(line, &mut test_logger);

        //Then
        assert!(test_logger.val.is_empty());
        assert_eq!(test_logger.error_val, "Invalid input at character 8");
    }

    #[test]
    fn is_variable_test() {
        //Given
        let line = "A=1";

        //When
        let is_var = is_variable(line);

        //Then
        assert!(is_var);
    }

    #[test]
    fn is_not_variable_test() {
        //Given
        let line = "3+3";

        //When
        let is_var = is_variable(line);

        //Then
        assert!(!is_var);
    }

    #[test]
    fn is_not_variable_test_2() {
        //Given
        let line = "A  =3";

        //When
        let is_var = is_variable(line);

        //Then
        assert!(!is_var);
    }

    #[test]
    fn is_not_variable_test_3() {
        //Given
        let line = "A+2";

        //When
        let is_var = is_variable(line);

        //Then
        assert!(!is_var);
    }

    #[test]
    fn bad_var_test() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();
        //B is not previously defined
        let line = "A=B";

        //When
        handle_var(line, &mut repl, &mut test_logger);
        //Then
        assert!(repl.variables.is_empty());
        assert!(test_logger.val.is_empty());
        assert_eq!(test_logger.error_val, "invalid variable value");
    }

    #[test]
    fn var_ans_test() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let line_1 = "1+1";
        let line_2 = "A=1";
        let line_3 = "ans + A";

        //When
        evaluate(line_1, &mut repl, &mut test_logger);
        //Then
        assert!(repl.previous_answer_valid);
        assert_eq!(repl.previous_answer, 2.0);
        assert_eq!(test_logger.val, "2.00");
        assert!(test_logger.error_val.is_empty());

        //When
        handle_var(line_2, &mut repl, &mut test_logger);
        //Then
        assert!(repl.variables.contains_key(&'A'));
        assert_eq!("1", repl.variables.get(&'A').unwrap());

        //When
        evaluate(line_3, &mut repl, &mut test_logger);

        //Then
        assert!(repl.previous_answer_valid);
        assert_eq!(repl.previous_answer, 3.0);
        assert_eq!(test_logger.val, "3.00");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn as_cli_tool_test_eval() {
        //Given
        let args = vec!["rmr".to_owned(), "-e".to_owned(), "3-sqrt(4)".to_owned()];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "1.00");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn as_cli_tool_test_eval_error() {
        //Given
        let args = vec!["rmr".to_owned(), "-e".to_owned(), "3-sqrt(4".to_owned()];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(test_logger.val.is_empty());
        assert_eq!(test_logger.error_val, "Invalid function");
    }

    #[test]
    fn as_cli_tool_test_eval_error_2() {
        //Given
        let args = vec!["rmr".to_owned(), "--evaluate".to_owned()];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(test_logger.val.is_empty());
        assert_eq!(test_logger.error_val, "Usage: rmr -e [expression]");
    }

    #[test]
    fn as_cli_tool_test_graph() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-g".to_owned(),
            "y=x".to_owned(),
            "-2".to_owned(),
            "2".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(is_graph_string(&test_logger.val));
        assert!(test_logger.error_val.is_empty());
    }
}
