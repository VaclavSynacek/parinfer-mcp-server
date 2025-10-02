# MCP Usage Guide

## What is MCP?

Model Context Protocol (MCP) is a standard for connecting AI assistants to external tools and data sources. This Parinfer MCP server allows AI assistants like Claude to fix Clojure code parentheses.

## Setup

### 1. Build the Server

```bash
make build
```

### 2. Configure Your MCP Client

#### Claude Desktop

Add to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`  
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "parinfer": {
      "command": "/absolute/path/to/parinfer-mcp/target/release/parinfer-mcp-server"
    }
  }
}
```

#### Other MCP Clients

Refer to your client's documentation for MCP server configuration. The server communicates via stdio using JSON-RPC.

## Testing

### Using MCP Inspector

The easiest way to test is with the official MCP inspector:

```bash
npx @modelcontextprotocol/inspector ./target/release/parinfer-mcp-server
```

This will open a web interface where you can:
1. See the server's capabilities
2. List available tools
3. Call tools with test inputs
4. Inspect requests and responses

### Using Integration Tests

Run the integration tests:

```bash
make test
```

### Manual JSON-RPC Testing

Send JSON-RPC requests directly to the server:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/parinfer-mcp-server
```

## Available Tools

### fix_parens

Fixes mismatched or missing closing parentheses in Clojure code based on indentation.

**Input:**
- `code` (string): Clojure code with correct indentation but potentially incorrect parentheses

**Output:**
- Corrected Clojure code with proper parentheses matching the indentation

**Example Usage in Claude:**

```
Please use the fix_parens tool to correct this Clojure code:

(defn calculate [x y
  (let [sum (+ x y
        diff (- x y
    {:sum sum
     :diff diff
```

Claude will call the tool and return:

```clojure
(defn calculate [x y]
  (let [sum (+ x y)
        diff (- x y)]
    {:sum sum
     :diff diff}))
```

## How It Works

The server:
1. Receives Clojure code with correct indentation
2. Uses the parinfer-rust library in "indent mode"
3. Infers correct closing parentheses/brackets based on indentation
4. Returns the corrected code

The indentation is preserved exactly as provided, and only the parentheses/brackets are fixed.

## Troubleshooting

### Server Won't Start

- Ensure the binary path in your config is absolute and correct
- Check that the binary has execute permissions: `chmod +x target/release/parinfer-mcp-server`
- Check logs in your MCP client for error messages

### Tool Not Available

- Restart your MCP client after configuration changes
- Verify the server starts successfully with the MCP inspector
- Check that the JSON configuration syntax is correct

### Unexpected Results

- Ensure input code has correct indentation
- The tool assumes indentation reflects the intended structure
- Test with the MCP inspector to verify behavior
