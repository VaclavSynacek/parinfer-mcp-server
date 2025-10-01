use std::process::{Command, Stdio};
use std::io::Write;
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

        let mut child = Command::new("cargo")
            .args(&["run", "--bin", "parinfer-cli"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn parinfer-cli");

        {
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(input.as_bytes()).expect("Failed to write to stdin");
        }

        let output = child.wait_with_output().expect("Failed to wait for child");
        let result = String::from_utf8(output.stdout).expect("Failed to parse stdout");

        assert_eq!(
            result, expected,
            "Mismatch for input file: {}\nExpected:\n{}\nGot:\n{}",
            input_file, expected, result
        );
    }
}
