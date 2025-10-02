# Parinfer MCP Server - Project Summary

## Overview

An MCP (Model Context Protocol) server that fixes Clojure code parentheses using the parinfer-rust library. It uses parinfer's "indent mode" to infer correct closing parentheses based on indentation.

## Project Structure

```
parinfer-mcp/
├── src/
│   └── main.rs              # MCP server implementation
├── tests/
│   └── integration_test.rs  # Integration tests
├── test-data/               # Test input/output pairs
│   ├── input-01.txt
│   ├── expected-01.txt
│   ├── input-02.txt
│   ├── expected-02.txt
│   ├── input-03.txt
│   └── expected-03.txt
├── Cargo.toml               # Rust project configuration
├── Cargo.lock               # Dependency lockfile
├── .gitignore              # Git ignore rules
├── README.md               # Main documentation
├── MCP_USAGE.md            # Detailed usage guide
└── SUMMARY.md              # This file
```

## Building

```bash
make build
```

Binary will be at `target/release/parinfer-mcp-server`.

## MCP Server

**Binary:** `target/release/parinfer-mcp-server`

**Protocol:** Model Context Protocol (JSON-RPC over stdio)

**Tool:** `fix_parens` - Accepts Clojure code, returns corrected code

**Configuration Example (Claude Desktop):**
```json
{
  "mcpServers": {
    "parinfer": {
      "command": "/absolute/path/to/parinfer-mcp-server"
    }
  }
}
```

**Testing:**
```bash
make test
npx @modelcontextprotocol/inspector ./target/release/parinfer-mcp-server
```

## Implementation Details

### Dependencies

- **parinfer_rust**: Git dependency from forked repository - Core parinfer functionality
- **rmcp**: MCP server SDK (v0.7 with server and transport-io features)
- **tokio**: Async runtime
- **serde/serde_json**: Serialization
- **anyhow**: Error handling
- **tracing**: Logging

### Key Features

- Preserves exact indentation
- Fixes mismatched parentheses/brackets
- Works with all Clojure/Lisp family languages
- Suitable for LLM code correction workflows

### Tests

Integration tests verify correct processing of all test cases in `test-data/` directory.

## Usage Example

When connected to an AI assistant:

```
User: Fix this Clojure code:
(defn calculate [x y
  (let [sum (+ x y
        diff (- x y
    {:sum sum
     :diff diff