# Parinfer CLI & MCP Server - Project Summary

## Overview

This project provides two ways to fix Clojure code parentheses using the parinfer-rust library:

1. **CLI Tool** (`parinfer-cli`) - Command-line tool for direct usage
2. **MCP Server** (`parinfer-mcp-server`) - Model Context Protocol server for AI assistant integration

Both tools use parinfer's "indent mode" to infer correct closing parentheses based on indentation.

## Project Structure

```
parinfer-mcp/
├── src/
│   ├── main.rs           # CLI tool implementation
│   └── mcp_server.rs     # MCP server implementation
├── tests/
│   └── integration_test.rs  # Integration tests
├── test-data/            # Test input/output pairs
│   ├── input-01.txt
│   ├── expected-01.txt
│   ├── input-02.txt
│   ├── expected-02.txt
│   ├── input-03.txt
│   └── expected-03.txt
├── parinfer-rust-lib/    # Cloned parinfer-rust library (modified)
├── Cargo.toml            # Rust project configuration
├── Makefile              # Build automation
├── .gitignore           # Git ignore rules
├── README.md            # Main documentation
├── MCP_USAGE.md         # MCP server usage guide
└── SUMMARY.md           # This file
```

## Building

```bash
make build        # Build both CLI and MCP server
make build-cli    # Build CLI only
make build-mcp    # Build MCP server only
make test         # Run tests
make clean        # Clean build artifacts
```

## CLI Tool

**Binary:** `target/release/parinfer-cli`

**Usage:**
```bash
cat broken.clj | parinfer-cli > fixed.clj
```

**Purpose:** Direct command-line usage for fixing Clojure code parentheses.

## MCP Server

**Binary:** `target/release/parinfer-mcp-server`

**Protocol:** Model Context Protocol (JSON-RPC over stdio)

**Tool:** `fix_parens` - Accepts Clojure code, returns corrected code

**Configuration Example (Claude Desktop):**
```json
{
  "mcpServers": {
    "parinfer": {
      "command": "/path/to/parinfer-mcp-server"
    }
  }
}
```

**Testing:**
```bash
npx @modelcontextprotocol/inspector ./target/release/parinfer-mcp-server
```

## Implementation Details

### Dependencies

- **parinfer_rust**: Local dependency (parinfer-rust-lib/) - Core parinfer functionality
- **rmcp**: MCP server SDK (v0.7 with server and transport-io features)
- **tokio**: Async runtime
- **serde/serde_json**: Serialization
- **tracing**: Logging

### Key Features

- Preserves exact indentation
- Fixes mismatched parentheses/brackets
- Works with all Clojure/Lisp family languages
- Suitable for LLM code correction workflows

### Tests

All tests pass:
- ✓ Integration tests with 3 test cases
- ✓ CLI verification
- ✓ MCP server initialization

## Usage Examples

### CLI Example

Input:
```clojure
(defn foo [x
  (+ x 1
```

Output:
```clojure
(defn foo [x]
  (+ x 1))
```

### MCP Example

When connected to an AI assistant:

```
User: Fix this Clojure code:
(defn calculate [x y
  (let [sum (+ x y
    sum