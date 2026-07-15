#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod rmr_tests {
    use rusty_maths::equation_analyzer::Definitions;

    use crate::modules::{
        bindings::{self, handle_let, looks_like_binding, LetSource},
        common::{GraphOptions, Point},
        error_render,
        evaluate::{evaluate, simple_evaluate},
        graphing::graph,
        logger::Logger,
        repl::Repl,
        run::as_cli_tool,
        string_maker::make_table_string,
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
        Repl::new(240)
    }

    fn empty_defs() -> Definitions {
        Definitions::new()
    }

    /// Run a `let` line as if typed at the prompt.
    fn let_line(line: &str, repl: &mut Repl, l: &mut TestLogger) -> bool {
        handle_let(line, repl, l, LetSource::Interactive)
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
        assert_eq!(repl.defs.value("ans"), Some(-7f32));
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
        assert_eq!(
            test_logger.error_val,
            format!(
                "{}{}^{}\nInvalid input",
                " ".repeat(10), // ">> " prompt (3) + span start (7)
                error_render::CARET_START,
                error_render::CARET_END
            )
        );
        // A failed evaluation never bound `ans`.
        assert_eq!(repl.defs.value("ans"), None);
    }

    #[test]
    fn evaluate_error_with_ans_carets_the_typed_line() {
        //Given: `ans` is bound from a previous answer
        let (mut repl, mut test_logger) = get_repl_and_logger();
        evaluate("3 + 4", &mut repl, &mut test_logger);

        //When: `ans` is a real binding now — no text substitution — so the
        //error span refers to the line exactly as typed
        evaluate("ans + foo(3)", &mut repl, &mut test_logger);

        //Then: the caret sits under `foo` in the echoed line (prompt 3 + 6)
        assert_eq!(
            test_logger.error_val,
            format!(
                "{}{}^^^{}\nInvalid function name foo",
                " ".repeat(9),
                error_render::CARET_START,
                error_render::CARET_END
            )
        );
        // And `ans` survives the failed evaluation.
        assert_eq!(repl.defs.value("ans"), Some(7.0));
    }

    #[test]
    fn graph_error_offsets_span_across_pipe_segments() {
        //Given a multi-equation graph where the second segment is invalid
        let go = get_graph_options();

        //When
        let result = graph("y=x|y=q", -1.0, 1.0, &go, &empty_defs());

        //Then the span maps onto the full entered text: `q` is at char 6
        let span = result.err().and_then(|e| e.span).map(|s| (s.start, s.end));
        assert_eq!(span, Some((6, 7)));
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
        assert_eq!(
            test_logger.error_val,
            format!(
                "(3+2+1)_2\n{}{}^{}\nInvalid input",
                " ".repeat(7), // span start
                error_render::CARET_START,
                error_render::CARET_END
            )
        );
    }

    // ============================================================================
    // `let` binding tests
    // ============================================================================

    #[test]
    fn let_value_binds_and_evaluates() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(let_line("let a = 3", &mut repl, &mut test_logger));
        assert_eq!(test_logger.val, "a = 3");
        assert_eq!(repl.defs.value("a"), Some(3.0));

        evaluate("a + 1", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "4");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn let_value_evaluates_eagerly() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(let_line("let a = 2 + 1", &mut repl, &mut test_logger));
        assert_eq!(repl.defs.value("a"), Some(3.0));

        // `k` captures a's value at definition time, not a reference.
        assert!(let_line("let k = a * 2", &mut repl, &mut test_logger));
        assert!(let_line("let a = 100", &mut repl, &mut test_logger));
        assert_eq!(repl.defs.value("k"), Some(6.0));
    }

    #[test]
    fn let_function_binds_and_evaluates() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(let_line("let g(x) = 2x^2", &mut repl, &mut test_logger));
        assert_eq!(test_logger.val, "g(x) = 2x^2");
        assert_eq!(repl.defs.function_body("g"), Some("2x^2"));

        evaluate("g(3)", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "18");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn let_function_resolves_values_late() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let a = 3", &mut repl, &mut test_logger);
        let_line("let g(x) = a * x", &mut repl, &mut test_logger);

        evaluate("g(2)", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "6");

        // Redefining `a` changes what g computes — late binding.
        let_line("let a = 5", &mut repl, &mut test_logger);
        evaluate("g(2)", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "10");
    }

    #[test]
    fn let_redefinition_notices_previous() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let a = 3", &mut repl, &mut test_logger);
        let_line("let a = 5", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "a = 5  (was 3)");

        let_line("let a(x) = x + 1", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "a(x) = x + 1  (was 5)");
        assert_eq!(repl.defs.value("a"), None);
        assert_eq!(repl.defs.function_body("a"), Some("x + 1"));
    }

    #[test]
    fn let_k_equals_ans_binds_the_value() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        evaluate("3 + 4", &mut repl, &mut test_logger);
        assert!(let_line("let k = ans", &mut repl, &mut test_logger));
        assert_eq!(test_logger.val, "k = 7");
        assert_eq!(repl.defs.value("k"), Some(7.0));
    }

    #[test]
    fn let_ans_is_rejected() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(!let_line("let ans = 3", &mut repl, &mut test_logger));
        assert!(test_logger.error_val.contains("set automatically"));
    }

    #[test]
    fn ans_banned_in_function_bodies() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        evaluate("3 + 4", &mut repl, &mut test_logger);
        assert!(!let_line("let g(x) = ans + x", &mut repl, &mut test_logger));
        assert!(test_logger.error_val.contains("bind it first: let k = ans"));
        assert!(!repl.defs.contains("g"));

        // ...but a *value* binding may use ans (evaluated eagerly), and a
        // body may then use that value.
        assert!(let_line("let k = ans", &mut repl, &mut test_logger));
        assert!(let_line("let g(x) = k + x", &mut repl, &mut test_logger));
    }

    #[test]
    fn let_function_requires_x_parameter() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(!let_line("let g(y) = 2y", &mut repl, &mut test_logger));
        assert!(test_logger.error_val.contains("must be exactly '(x)'"));
    }

    #[test]
    fn let_rejects_catalog_and_reserved_names() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(!let_line("let sin = 3", &mut repl, &mut test_logger));
        assert!(test_logger.error_val.contains("built-in"));

        assert!(!let_line("let x = 3", &mut repl, &mut test_logger));
        assert!(test_logger.error_val.contains("reserved"));
    }

    #[test]
    fn let_broken_body_keeps_old_binding() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let g(x) = x + 1", &mut repl, &mut test_logger);
        // `nope` is unknown, so the new body fails validation...
        assert!(!let_line(
            "let g(x) = nope * x",
            &mut repl,
            &mut test_logger
        ));
        assert!(test_logger.error_val.contains("Unknown name 'nope'"));
        // ...and the working definition survives.
        assert_eq!(repl.defs.function_body("g"), Some("x + 1"));
    }

    #[test]
    fn let_value_rhs_error_carets_typed_line() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(!let_line("let a = foo(3)", &mut repl, &mut test_logger));
        //          let a = foo(3)
        // prompt(3) + offset of `foo` in the line (8) = 11
        assert_eq!(
            test_logger.error_val,
            format!(
                "{}{}^^^{}\nInvalid function name foo",
                " ".repeat(11),
                error_render::CARET_START,
                error_render::CARET_END
            )
        );
    }

    #[test]
    fn let_rejects_non_finite_values() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        assert!(!let_line("let a = 1/0", &mut repl, &mut test_logger));
        assert!(test_logger.error_val.contains("finite"));
        assert!(!repl.defs.contains("a"));
    }

    #[test]
    fn call_time_body_error_reprints_body() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let a = 3", &mut repl, &mut test_logger);
        let_line("let g(x) = a * x", &mut repl, &mut test_logger);
        bindings::undefine("a", &mut repl, &mut test_logger);

        //When: g's body no longer resolves at call time
        evaluate("g(2)", &mut repl, &mut test_logger);

        //Then: the error reprints the definition and carets into the body
        assert_eq!(
            test_logger.error_val,
            format!(
                "in g(x) = a * x\n{}{}^{}\nUnknown name 'a'",
                " ".repeat(10),
                error_render::CARET_START,
                error_render::CARET_END
            )
        );
    }

    #[test]
    fn undefine_removes_bindings() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let a = 3", &mut repl, &mut test_logger);
        bindings::undefine("a", &mut repl, &mut test_logger);
        assert!(!repl.defs.contains("a"));
        assert_eq!(test_logger.val, "a removed");

        bindings::undefine("a", &mut repl, &mut test_logger);
        assert!(test_logger.error_val.contains("Nothing named 'a'"));

        bindings::undefine("ans", &mut repl, &mut test_logger);
        assert!(test_logger.error_val.contains("maintained automatically"));
    }

    #[test]
    fn looks_like_binding_detects_missing_let() {
        assert!(looks_like_binding("a = 3"));
        assert!(looks_like_binding("g(x) = 2x^2"));
        assert!(looks_like_binding("rate = 0.07"));

        // Equation syntax and plain math are not bindings.
        assert!(!looks_like_binding("y = 2x"));
        assert!(!looks_like_binding("x = 3"));
        assert!(!looks_like_binding("3 + 3"));
        assert!(!looks_like_binding("2 ^ 3"));
    }

    #[test]
    fn undef_command_routes_through_run_command() {
        use crate::modules::commands::run_command;
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let a = 3", &mut repl, &mut test_logger);
        run_command("undef a", &mut test_logger, &mut repl);
        assert_eq!(test_logger.val, "a removed");
        assert!(!repl.defs.contains("a"));

        run_command("undef", &mut test_logger, &mut repl);
        assert!(test_logger.error_val.contains("Usage: :undef"));
    }

    #[test]
    fn fns_shows_user_bindings() {
        use crate::modules::commands::run_command;
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let g(x) = 2x^2", &mut repl, &mut test_logger);

        // `:fns g` describes the binding...
        run_command("fns g", &mut test_logger, &mut repl);
        assert!(test_logger.val.contains("function binding"));

        // ...and `:fns <catalog name>` still works. (TestLogger keeps only
        // the last printed line — the example line for a catalog entry.)
        run_command("fns sin", &mut test_logger, &mut repl);
        assert!(test_logger.val.contains("sin(0) = 0"));
    }

    #[test]
    fn piped_user_function_evaluates() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let g(x) = 2x^2", &mut repl, &mut test_logger);
        evaluate("4 |> g", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "32");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn table_mode_error_reprints_body_for_broken_binding() {
        // The `:t`/`:g` flows render through render_error_with_source; a
        // body-tagged error must reprint the definition, not the equation.
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let_line("let a = 3", &mut repl, &mut test_logger);
        let_line("let g(x) = a * x", &mut repl, &mut test_logger);
        bindings::undefine("a", &mut repl, &mut test_logger);

        let go = get_graph_options();
        let err = graph("y = g(x)", -1.0, 1.0, &go, &repl.defs).unwrap_err();
        let rendered = error_render::render_error_with_source("y = g(x)", &err, &repl.defs);
        assert!(rendered.starts_with("in g(x) = a * x\n"));
        assert!(rendered.contains("Unknown name 'a'"));
    }

    #[test]
    fn recursive_function_errors_cleanly() {
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let g(x) = g(x)", &mut repl, &mut test_logger);
        evaluate("g(1)", &mut repl, &mut test_logger);
        assert!(test_logger.error_val.contains("Call depth limit"));
    }

    #[test]
    fn bindings_persist_and_reload() {
        let path = std::env::temp_dir().join(format!("rmr_bindings_test_{}", std::process::id()));
        let _ = std::fs::remove_file(&path);

        //Given a repl that persists to a temp file
        let (mut repl, mut test_logger) = get_repl_and_logger();
        repl.bindings_path = Some(path.clone());

        let_line("let a = 3", &mut repl, &mut test_logger);
        let_line("let g(x) = a * x^2", &mut repl, &mut test_logger);
        evaluate("21 + 21", &mut repl, &mut test_logger); // ans must NOT persist

        let saved = std::fs::read_to_string(&path).unwrap_or_default();
        assert_eq!(saved, "let a = 3\nlet g(x) = a * x^2\n");

        //When a fresh repl loads the same file
        let mut repl2 = get_repl();
        repl2.bindings_path = Some(path.clone());
        bindings::load(&mut repl2, &mut test_logger);

        //Then the bindings are back and work
        assert_eq!(repl2.defs.value("a"), Some(3.0));
        assert_eq!(repl2.defs.function_body("g"), Some("a * x^2"));
        assert_eq!(repl2.defs.value("ans"), None);
        evaluate("g(2)", &mut repl2, &mut test_logger);
        assert_eq!(test_logger.val, "12");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn broken_persisted_line_warns_and_skips() {
        let path =
            std::env::temp_dir().join(format!("rmr_bindings_bad_test_{}", std::process::id()));
        std::fs::write(&path, "let a = 3\nnot a let line\nlet b = 4\n").unwrap_or_default();

        let (mut repl, mut test_logger) = get_repl_and_logger();
        repl.bindings_path = Some(path.clone());
        bindings::load(&mut repl, &mut test_logger);

        // The good lines loaded; the bad one warned.
        assert_eq!(repl.defs.value("a"), Some(3.0));
        assert_eq!(repl.defs.value("b"), Some(4.0));
        assert!(test_logger.error_val.contains("ignored line 2"));

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn graph_sees_user_functions() {
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let_line("let g(x) = 2x^2", &mut repl, &mut test_logger);

        let go = get_graph_options();
        let g = graph("y = g(x)", -2.0, 2.0, &go, &repl.defs);
        assert!(g.is_ok());
        assert!(is_graph_string(&g.unwrap()));
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
        assert_eq!(
            test_logger.error_val,
            format!(
                "3-sqrt(4\n  {}^^^^^{}\nInvalid function",
                error_render::CARET_START,
                error_render::CARET_END
            )
        );
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
        assert_eq!(
            format!(
                "y=q\n  {}^{}\nUnknown name 'q'",
                error_render::CARET_START,
                error_render::CARET_END
            ),
            test_logger.error_val
        );
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
        assert_eq!(
            "Invalid use of rmr. Usage: rmr [expression] or rmr -g/-t [args]",
            test_logger.error_val
        );
    }

    #[test]
    fn graph_test() {
        //Given
        let eq_str = "y =tan(x  )";
        let x_min = -5f32;
        let x_max = 5f32;

        let go = get_graph_options();

        //When
        let g = graph(eq_str, x_min, x_max, &go, &empty_defs());

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
        assert_eq!(
            format!(
                "y=q\n  {}^{}\nUnknown name 'q'",
                error_render::CARET_START,
                error_render::CARET_END
            ),
            test_logger.error_val
        );
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
    // Binding names flow through the real tokenizer (no text substitution)
    // ============================================================================

    #[test]
    fn binding_does_not_corrupt_function_names() {
        //Given a binding whose name appears inside catalog names
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let_line("let s = 5", &mut repl, &mut test_logger);
        let_line("let a = 100", &mut repl, &mut test_logger);

        //When using functions containing those letters
        evaluate("sin(1)", &mut repl, &mut test_logger);
        assert!(test_logger.val.contains("0.8")); // sin(1) ≈ 0.841, not 5in(1)

        evaluate("tan(1)", &mut repl, &mut test_logger);
        assert!(test_logger.val.contains("1.5")); // tan(1) ≈ 1.557

        evaluate("max(s, a)", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "100");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn binding_in_standalone_context() {
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let_line("let a = 42", &mut repl, &mut test_logger);

        evaluate("a", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "42");
        assert!(test_logger.error_val.is_empty());
    }

    #[test]
    fn binding_in_expression_with_operators() {
        let (mut repl, mut test_logger) = get_repl_and_logger();
        let_line("let b = 7", &mut repl, &mut test_logger);

        evaluate("b+b*b", &mut repl, &mut test_logger);
        assert_eq!(test_logger.val, "56"); // 7 + 7*7
        assert!(test_logger.error_val.is_empty());
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
        let g = graph(eq_str, x_min, x_max, &go, &empty_defs());

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
        let g = graph(eq_str, x_min, x_max, &go, &empty_defs());

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

        //When - remove a binding that doesn't exist
        bindings::undefine("zz", &mut repl, &mut test_logger);

        //Then - error message should start with capital letter
        assert!(test_logger.error_val.starts_with("Nothing"));
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
        let g = graph(eq_str, x_min, x_max, &go, &empty_defs());

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
    fn multiple_bindings_in_expression() {
        //Given
        let (mut repl, mut test_logger) = get_repl_and_logger();

        let_line("let a = 5", &mut repl, &mut test_logger);
        let_line("let b = 3", &mut repl, &mut test_logger);

        //When
        evaluate("a*b+a", &mut repl, &mut test_logger);

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
        let g = graph(eq_str, x_min, x_max, &go, &empty_defs());

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
        let g = graph(eq_str, x_min, x_max, &go, &empty_defs());

        //Then - should return an error
        assert!(g.is_err());
    }
}
