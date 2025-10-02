use std::fs;

#[test]
fn test_all_parinfer_cases() {
    let test_cases = vec![
        ("test-data/input-01.txt", "test-data/expected-01.txt"),
        ("test-data/input-02.txt", "test-data/expected-02.txt"),
        ("test-data/input-03.txt", "test-data/expected-03.txt"),
    ];

    for (input_file, expected_file) in test_cases {
        let input = fs::read_to_string(input_file)
            .expect(&format!("Failed to read {}", input_file));
        let expected = fs::read_to_string(expected_file)
            .expect(&format!("Failed to read {}", expected_file));

        let options = parinfer_rust::types::Options {
            cursor_x: None,
            cursor_line: None,
            prev_cursor_x: None,
            prev_cursor_line: None,
            prev_text: None,
            selection_start_line: None,
            changes: vec![],
            comment_char: ';',
            string_delimiters: vec!["\"".to_string()],
            lisp_vline_symbols: false,
            lisp_block_comments: false,
            guile_block_comments: false,
            scheme_sexp_comments: false,
            janet_long_strings: false,
            hy_bracket_strings: false,
        };

        let answer = parinfer_rust::parinfer::indent_mode(&input, &options);

        assert!(
            answer.success,
            "Parinfer failed for input file: {}\nError: {:?}",
            input_file, answer.error
        );

        assert_eq!(
            answer.text, expected,
            "Mismatch for input file: {}\nExpected:\n{}\nGot:\n{}",
            input_file, expected, answer.text
        );
    }
}
