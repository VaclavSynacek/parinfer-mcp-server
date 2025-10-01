# Parinfer CLI

A command-line tool that uses the parinfer-rust library to automatically fix parentheses and brackets in Clojure code based on indentation.

## Purpose

This tool is designed to help LLMs correct Clojure code when parentheses are mismatched or incorrectly nested. It takes Clojure code with correct indentation but potentially incorrect parentheses/brackets on standard input and outputs corrected code.

## Building

```bash
cargo build --release
```

The binary will be available at `target/release/parinfer-cli`.

## Usage

```bash
cat input.clj | parinfer-cli > output.clj
```

Or:

```bash
parinfer-cli < input.clj > output.clj
```

## How It Works

The tool uses parinfer's "indent mode" which:
1. Preserves all indentation exactly as provided
2. Infers correct closing parentheses/brackets based on indentation
3. Fixes mismatched or missing closing delimiters

## Testing

Run the integration tests:

```bash
cargo test
```

The tests verify that the tool correctly processes all test cases in the `test-data/` directory.

## Example

Input with incorrect parentheses:
```clojure
(defn nnbsp [n
  (apply str (repeat n nbsp))
```

Output with corrected parentheses:
```clojure
(defn nnbsp [n]
  (apply str (repeat n nbsp)))
```
