# Parinfer MCP Server

An MCP (Model Context Protocol) server that uses the parinfer-rust library to automatically fix parentheses and brackets in Clojure code based on indentation.

## Purpose

This server is designed to help LLMs correct Clojure code when parentheses are mismatched or incorrectly nested. It takes Clojure code with correct indentation but potentially incorrect parentheses/brackets and outputs corrected code.

## Building

```bash
make build
```

The binary will be available at `target/release/parinfer-mcp-server`.

## Usage

The MCP server can be used with any MCP-compatible client (like Claude Desktop, Cline, etc.).

### Configuration

Add to your MCP client configuration:

**Claude Desktop** (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):
```json
{
  "mcpServers": {
    "parinfer": {
      "command": "/absolute/path/to/parinfer-mcp/target/release/parinfer-mcp-server"
    }
  }
}
```

For other platforms and clients, see [MCP_USAGE.md](MCP_USAGE.md).

### Available Tools

The MCP server provides one tool:

- **fix_parens**: Fixes mismatched or missing closing parentheses in Clojure code based on indentation
  - Input: `code` (string) - Clojure code with correct indentation
  - Output: Corrected Clojure code with proper parentheses

## How It Works

The server uses parinfer's "indent mode" which:
1. Preserves all indentation exactly as provided
2. Infers correct closing parentheses/brackets based on indentation
3. Fixes mismatched or missing closing delimiters

## Testing

Run tests:
```bash
make test
```

Test with MCP Inspector:
```bash
npx @modelcontextprotocol/inspector ./target/release/parinfer-mcp-server
```

See [MCP_USAGE.md](MCP_USAGE.md) for detailed testing instructions.

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
