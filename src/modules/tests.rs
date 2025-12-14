#[cfg(test)]
mod rmr_tests {
    use std::collections::HashMap;

    use crate::modules::{
        common::{GraphOptions, Point},
        evaluate::{evaluate, simple_evaluate},
        graphing::graph,
        logger::Logger,
        repl::Repl,
        run::as_cli_tool,
        string_maker::make_table_string,
        variables::{handle_var, is_variable},
    };

    pub(crate) struct TestLogger {
        pub(crate) val: String,
        pub(crate) error_val: String,
    }

    impl Logger for TestLogger {
        fn print(&mut self, value: &str) {
            value.clone_into(&mut self.val);
        }
        fn eprint(&mut self, value: &str) {
            value.clone_into(&mut self.error_val);
        }
    }

    fn get_repl() -> Repl {
        Repl {
            previous_answer: 0.0,
            previous_answer_valid: false,
            variables: HashMap::new(),
            width: 240,
            height: 120,
        }
    }

    fn get_test_logger() -> TestLogger {
        TestLogger {
            val: String::new(),
            error_val: String::new(),
        }
    }

    fn get_graph_options() -> GraphOptions {
        GraphOptions {
            y_max: 7.,
            y_min: -7.,
            width: 240,
            height: 120,
        }
    }

    fn get_repl_and_logger() -> (Repl, TestLogger) {
        (get_repl(), get_test_logger())
    }

    fn is_graph_string(g: &str) -> bool {
        //The only place in the program these three chars are used is in the creation of a valid graph
        let empty_braille_char = '⠀';
        let upper_left = '┌';
        let upper_right = '┐';

        g.contains(empty_braille_char) && g.contains(upper_left) && g.contains(upper_right)
    }

    fn is_table_string(g: &str) -> bool {
        //Table uses the same upper left and right but no braille
        let empty_braille_char = '⠀';
        let upper_left = '┌';
        let upper_right = '┐';

        !g.contains(empty_braille_char) && g.contains(upper_left) && g.contains(upper_right)
    }

    #[test]
    fn evaluate_test() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let line = "max(2,3)-min(10,11)";

        //When
        evaluate(line, &mut repl, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "-7");
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
        assert_eq!(test_logger.val, "3");
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
        assert_eq!(test_logger.error_val, "Invalid variable value");
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
        assert_eq!(test_logger.val, "2");
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
        assert_eq!(test_logger.val, "3");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn as_cli_tool_test_eval() {
        //Given
        let args = vec!["rmr".to_owned(), "3-sqrt(4)".to_owned()];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "1");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn as_cli_tool_test_eval_error() {
        //Given
        let args = vec!["rmr".to_owned(), "3-sqrt(4".to_owned()];
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
        let args = vec!["rmr".to_owned()];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(test_logger.val.is_empty());
        assert_eq!(test_logger.error_val, "Usage: rmr [expression]");
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

    #[test]
    fn as_cli_tool_test_graph_error() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-g".to_owned(),
            "y=x".to_owned(),
            "-2".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!(
            "Usage: rmr -g [equation] [x-min] [x-max]",
            test_logger.error_val
        );
    }

    #[test]
    fn as_cli_tool_test_graph_error_2() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-g".to_owned(),
            "y=x".to_owned(),
            "-2".to_owned(),
            "-3".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!(
            "x min `-2` must be less than x max `-3`",
            test_logger.error_val
        );
    }

    #[test]
    fn as_cli_tool_test_graph_error_3() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-g".to_owned(),
            "y=q".to_owned(),
            "-2".to_owned(),
            "5".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!("Invalid input at character 2", test_logger.error_val);
    }

    #[test]
    fn as_cli_tool_test_graph_error_4() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "--graph".to_owned(),
            "y=q".to_owned(),
            "-2".to_owned(),
            "a".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!(
            "x-min: `-2` and x-max: `a` must both be valid numbers",
            test_logger.error_val
        );
    }

    #[test]
    fn as_cli_tool_test_bad_flag() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-69".to_owned(),
            "y=q".to_owned(),
            "-2".to_owned(),
            "a".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!("Invalid use of rmr. Usage: rmr [expression] or rmr -g/-t [args]", test_logger.error_val);
    }

    #[test]
    fn graph_test() {
        //Given
        let eq_str = "y =tan(x  )";
        let x_min = -5f32;
        let x_max = 5f32;

        let go = get_graph_options();

        //When
        let g = graph(eq_str, x_min, x_max, &go);

        //Then
        assert!(g.is_ok());
        assert!(is_graph_string(&g.unwrap()));
    }

    #[test]
    fn make_table_test() {
        //Given
        let points = vec![
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
            Point::new(3.0, 3.0),
        ];

        //When
        let table_string = make_table_string(points);

        //Then
        assert!(is_table_string(&table_string));
    }

    #[test]
    fn as_cli_tool_test_table() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-t".to_owned(),
            "y=x".to_owned(),
            "-2".to_owned(),
            "2".to_owned(),
            "1".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(is_table_string(&test_logger.val));
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn as_cli_tool_test_table_error() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-t".to_owned(),
            "y=x".to_owned(),
            "-2".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!(
            "Usage: rmr -t [equation] [x-min] [x-max] [step_size]",
            test_logger.error_val
        );
    }

    #[test]
    fn as_cli_tool_test_graph_table_2() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-t".to_owned(),
            "y=x".to_owned(),
            "-2".to_owned(),
            "-3".to_owned(),
            "1".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!(
            "x min `-2` must be less than x max `-3`",
            test_logger.error_val
        );
    }

    #[test]
    fn as_cli_tool_test_table_error_3() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "-t".to_owned(),
            "y=q".to_owned(),
            "-2".to_owned(),
            "5".to_owned(),
            "1".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!("Invalid input at character 2", test_logger.error_val);
    }

    #[test]
    fn as_cli_tool_test_table_error_4() {
        //Given
        let args = vec![
            "rmr".to_owned(),
            "--table".to_owned(),
            "y=q".to_owned(),
            "-2".to_owned(),
            "a".to_owned(),
            "1".to_owned(),
        ];
        let mut test_logger = get_test_logger();

        //When
        as_cli_tool(&args, &mut test_logger);

        //Then
        assert!(&test_logger.val.is_empty());
        assert_eq!(
            "x-min: `-2`, x-max: `a` and step_size: `1` must all be valid numbers",
            test_logger.error_val
        );
    }

    // ============================================================================
    // Variable Replacement Word Boundary Tests (Critical Bug Fix Verification)
    // ============================================================================

    #[test]
    fn variable_does_not_replace_in_function_names() {
        //Given - variable 'S' should not replace 's' in 'sin'
        let (mut repl, mut test_logger) = get_repl_and_logger();

        handle_var("S=5", &mut repl, &mut test_logger);

        //When - use 's' in a function name
        evaluate("sin(1)", &mut repl, &mut test_logger);

        //Then - should evaluate sin(1), not 5in(1)
        assert!(test_logger.val.contains("0.8")); // sin(1) ≈ 0.841
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn variable_replacement_respects_word_boundaries() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        handle_var("X=10", &mut repl, &mut test_logger);

        //When - X in "X+5" should be replaced, but x in "max" should not
        evaluate("max(X, 5)", &mut repl, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "10"); // max(10, 5) = 10
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn variable_in_standalone_context() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        handle_var("A=42", &mut repl, &mut test_logger);

        //When - just the variable by itself
        evaluate("A", &mut repl, &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "42");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn variable_in_expression_with_operators() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        handle_var("B=7", &mut repl, &mut test_logger);

        //When - variable with operators around it
        evaluate("B+B*B", &mut repl, &mut test_logger);

        //Then - 7 + 7*7 = 7 + 49 = 56
        assert_eq!(test_logger.val, "56");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn variable_does_not_replace_in_concatenated_functions() {
        //Given - variable A should not interfere with function names
        let (mut repl, mut test_logger) = get_repl_and_logger();

        handle_var("A=100", &mut repl, &mut test_logger);

        //When - using tan function (has 'a' in it)
        evaluate("tan(1)", &mut repl, &mut test_logger);

        //Then - should evaluate tan(1), not t100n(1)
        assert!(test_logger.val.contains("1.5")); // tan(1) ≈ 1.557
        assert!(test_logger.error_val.is_empty());
    }

    // ============================================================================
    // Variable Detection Edge Case Tests
    // ============================================================================

    #[test]
    fn is_variable_lowercase_first_char() {
        //Given
        let line = "a=1";

        //When
        let result = is_variable(line);

        //Then - should be false, must start with uppercase
        assert!(!result);
    }

    #[test]
    fn is_variable_number_first_char() {
        //Given
        let line = "1A=5";

        //When
        let result = is_variable(line);

        //Then - should be false
        assert!(!result);
    }

    #[test]
    fn is_variable_special_char_first() {
        //Given
        let line = "$A=5";

        //When
        let result = is_variable(line);

        //Then - should be false
        assert!(!result);
    }

    #[test]
    fn is_variable_with_space_after_name() {
        //Given
        let line = "A =5";

        //When
        let result = is_variable(line);

        //Then - should be true
        assert!(result);
    }

    #[test]
    fn is_variable_no_equals() {
        //Given
        let line = "ABC";

        //When
        let result = is_variable(line);

        //Then - should be false, no = sign
        assert!(!result);
    }

    #[test]
    fn is_variable_too_short() {
        //Given
        let line = "A";

        //When
        let result = is_variable(line);

        //Then - should be false, need at least "A="
        assert!(!result);
    }

    #[test]
    fn is_variable_multiple_equals() {
        //Given
        let line = "A=1=2";

        //When
        let result = is_variable(line);

        //Then - should still be true (first = is what matters)
        assert!(result);
    }

    // ============================================================================
    // Nested Variadic Function Tests (rusty-maths upgrade showcase)
    // ============================================================================

    #[test]
    fn nested_variadic_max_min() {
        //Given
        let mut test_logger = get_test_logger();

        //When - nested max and min functions
        simple_evaluate("max(min(10,20),5,max(3,7))", &mut test_logger);

        //Then - min(10,20)=10, max(3,7)=7, max(10,5,7)=10
        assert_eq!(test_logger.val, "10");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn deeply_nested_functions() {
        //Given
        let mut test_logger = get_test_logger();

        //When
        simple_evaluate("max(1,min(5,max(2,3)))", &mut test_logger);

        //Then - max(2,3)=3, min(5,3)=3, max(1,3)=3
        assert_eq!(test_logger.val, "3");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn nested_min_functions() {
        //Given
        let mut test_logger = get_test_logger();

        //When
        simple_evaluate("min(min(10,5),min(8,3))", &mut test_logger);

        //Then - min(10,5)=5, min(8,3)=3, min(5,3)=3
        assert_eq!(test_logger.val, "3");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn nested_with_arithmetic() {
        //Given
        let mut test_logger = get_test_logger();

        //When
        simple_evaluate("max(2+3, min(10,7))", &mut test_logger);

        //Then - 2+3=5, min(10,7)=7, max(5,7)=7
        assert_eq!(test_logger.val, "7");
        assert!(test_logger.error_val.is_empty());
    }

    // ============================================================================
    // Multiple Equation Graph Tests
    // ============================================================================

    #[test]
    fn graph_multiple_equations() {
        //Given - two equations separated by |
        let eq_str = "y=x|y=2*x";
        let x_min = -2f32;
        let x_max = 2f32;
        let go = get_graph_options();

        //When
        let g = graph(eq_str, x_min, x_max, &go);

        //Then - should produce a valid graph with both equations
        assert!(g.is_ok());
        assert!(is_graph_string(&g.unwrap()));
    }

    #[test]
    fn graph_three_equations() {
        //Given
        let eq_str = "y=x|y=x^2|y=x^3";
        let x_min = -1f32;
        let x_max = 1f32;
        let go = get_graph_options();

        //When
        let g = graph(eq_str, x_min, x_max, &go);

        //Then
        assert!(g.is_ok());
        assert!(is_graph_string(&g.unwrap()));
    }

    // ============================================================================
    // Bezier Curve Tests
    // ============================================================================

    #[test]
    fn quadratic_bezier_renders() {
        //Given
        let mut test_logger = get_test_logger();
        let go = get_graph_options();

        let p1 = Point::new(10.0, 10.0);
        let p2 = Point::new(50.0, 80.0);
        let p3 = Point::new(90.0, 10.0);

        //When
        use crate::modules::bezier_curve::quadratic_bezier;
        quadratic_bezier(p1, p2, p3, &go, &mut test_logger);

        //Then - should produce output
        assert!(!test_logger.val.is_empty());
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn cubic_bezier_renders() {
        //Given
        let mut test_logger = get_test_logger();
        let go = get_graph_options();

        let p1 = Point::new(10.0, 10.0);
        let p2 = Point::new(30.0, 80.0);
        let p3 = Point::new(70.0, 80.0);
        let p4 = Point::new(90.0, 10.0);

        //When
        use crate::modules::bezier_curve::cubic_bezier;
        cubic_bezier(p1, p2, p3, p4, &go, &mut test_logger);

        //Then
        assert!(!test_logger.val.is_empty());
        assert!(test_logger.error_val.is_empty());
    }

    // ============================================================================
    // Error Message Improvement Tests
    // ============================================================================

    #[test]
    fn invalid_command_shows_help_hint() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        //When
        use crate::modules::commands::run_command;
        run_command("invalidcmd", &mut test_logger, &mut repl);

        //Then
        assert!(test_logger.error_val.contains("Type ':h' for help"));
    }

    #[test]
    fn error_messages_are_capitalized() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        //When - try to use undefined variable in variable assignment
        handle_var("A=B", &mut repl, &mut test_logger);

        //Then - error message should start with capital letter
        assert!(test_logger.error_val.starts_with("Invalid"));
    }

    // ============================================================================
    // Edge Case Tests
    // ============================================================================

    #[test]
    fn evaluate_with_extra_whitespace() {
        //Given
        let mut test_logger = get_test_logger();

        //When - expression with extra spaces
        simple_evaluate("  2   +   2  ", &mut test_logger);

        //Then
        assert_eq!(test_logger.val, "4");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn graph_with_whitespace_in_equation() {
        //Given
        let eq_str = "y = x ^ 2";
        let x_min = -2f32;
        let x_max = 2f32;
        let go = get_graph_options();

        //When
        let g = graph(eq_str, x_min, x_max, &go);

        //Then - should handle whitespace
        assert!(g.is_ok());
    }

    #[test]
    fn division_by_zero_in_evaluation() {
        //Given
        let mut test_logger = get_test_logger();

        //When
        simple_evaluate("1/0", &mut test_logger);

        //Then - should either error or return inf (depending on rusty-maths behavior)
        // We just verify it doesn't panic
        assert!(!test_logger.val.is_empty() || !test_logger.error_val.is_empty());
    }

    #[test]
    fn very_large_numbers() {
        //Given
        let mut test_logger = get_test_logger();

        //When
        simple_evaluate("999999*999999", &mut test_logger);

        //Then - should handle large numbers
        assert!(!test_logger.val.is_empty());
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn very_small_numbers() {
        //Given
        let mut test_logger = get_test_logger();

        //When
        simple_evaluate("0.000001*0.000001", &mut test_logger);

        //Then
        assert!(!test_logger.val.is_empty());
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn negative_numbers_in_expressions() {
        //Given
        let mut test_logger = get_test_logger();

        //When
        simple_evaluate("-5*-3", &mut test_logger);

        //Then - should be 15
        assert_eq!(test_logger.val, "15");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn multiple_variables_in_expression() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        handle_var("A=5", &mut repl, &mut test_logger);
        handle_var("B=3", &mut repl, &mut test_logger);

        //When
        evaluate("A*B+A", &mut repl, &mut test_logger);

        //Then - 5*3+5 = 20
        assert_eq!(test_logger.val, "20");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn ans_with_multiple_operations() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        //When - chain of operations using ans
        evaluate("10", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "10");

        evaluate("ans*2", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "20");

        evaluate("ans+5", &mut repl, &mut test_logger);

        //Then - should be 25
        assert_eq!(test_logger.val, "25");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn empty_equation_in_graph() {
        //Given
        let eq_str = "";
        let x_min = -1f32;
        let x_max = 1f32;
        let go = get_graph_options();

        //When
        let g = graph(eq_str, x_min, x_max, &go);

        //Then - should return an error
        assert!(g.is_err());
    }

    #[test]
    fn graph_with_invalid_equation() {
        //Given
        let eq_str = "y=invalid";
        let x_min = -1f32;
        let x_max = 1f32;
        let go = get_graph_options();

        //When
        let g = graph(eq_str, x_min, x_max, &go);

        //Then - should return an error
        assert!(g.is_err());
    }
}
