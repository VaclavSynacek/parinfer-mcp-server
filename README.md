# Parinfer CLI & MCP Server

A command-line tool and MCP (Model Context Protocol) server that uses the parinfer-rust library to automatically fix parentheses and brackets in Clojure code based on indentation.

## Purpose

These tools are designed to help LLMs correct Clojure code when parentheses are mismatched or incorrectly nested. They take Clojure code with correct indentation but potentially incorrect parentheses/brackets and output corrected code.

## Building

Build everything:
```bash
make build
```

Or build individually:
```bash
make build-cli   # CLI tool only
make build-mcp   # MCP server only
```

Or with cargo directly:
```bash
cargo build --release
```

Binaries will be available at:
- `target/release/parinfer-cli` - CLI tool
- `target/release/parinfer-mcp-server` - MCP server

## CLI Usage

```bash
cat input.clj | parinfer-cli > output.clj
```

Or:

```bash
parinfer-cli < input.clj > output.clj
```

## MCP Server Usage

The MCP server can be used with any MCP-compatible client (like Claude Desktop, Cline, etc.).

### Testing with MCP Inspector

```bash
npx @modelcontextprotocol/inspector ./target/release/parinfer-mcp-server
```

### Configuration

Add to your MCP client configuration (e.g., Claude Desktop):

```json
{
  "mcpServers": {
    "parinfer": {
      "command": "/path/to/parinfer-mcp/target/release/parinfer-mcp-server"
    }
  }
}
```

### Available Tools

The MCP server provides one tool:

- **fix_parens**: Fixes mismatched or missing closing parentheses in Clojure code based on indentation
  - Input: `code` (string) - Clojure code with correct indentation
  - Output: Corrected Clojure code with proper parentheses

## How It Works

Both tools use parinfer's "indent mode" which:
1. Preserves all indentation exactly as provided
2. Infers correct closing parentheses/brackets based on indentation
3. Fixes mismatched or missing closing delimiters

## Testing

Run all tests:

```bash
make test
```

Or with cargo:

```bash
cargo test
```

The tests verify that both the CLI and MCP server correctly process all test cases in the `test-data/` directory.

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
