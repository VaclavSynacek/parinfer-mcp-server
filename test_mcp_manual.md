# Testing the Parinfer MCP Server

## Using MCP Inspector

The easiest way to test the MCP server is with the MCP Inspector:

```bash
npx @modelcontextprotocol/inspector ./target/debug/parinfer-mcp-server
```

Or with the release build:

```bash
npx @modelcontextprotocol/inspector ./target/release/parinfer-mcp-server
```

## Manual Testing with JSON-RPC

You can also test by sending JSON-RPC messages directly to stdin:

### 1. Initialize

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}
```

### 2. List Tools

```json
{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
```

### 3. Call fix_parens Tool

```json
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"fix_parens","arguments":{"code":"(defn foo [x\n  (+ x 1"}}}
```

## Example Test Code

Input with broken parentheses:
```clojure
(defn nnbsp [n
  (apply str (repeat n nbsp))
```

Expected output:
```clojure
(defn nnbsp [n]
  (apply str (repeat n nbsp)))
```
