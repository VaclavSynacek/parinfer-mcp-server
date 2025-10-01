use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

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
    
    if answer.success {
        print!("{}", answer.text);
    } else {
        eprintln!("Error: {:?}", answer.error);
        std::process::exit(1);
    }
}
