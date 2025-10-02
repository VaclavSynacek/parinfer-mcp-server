#!/bin/bash

# Test the MCP server by sending JSON-RPC requests

SERVER="./target/debug/parinfer-mcp-server"

# Function to send a JSON-RPC request
send_request() {
    echo "$1" | $SERVER 2>/dev/null
}

echo "=== Testing Parinfer MCP Server ==="
echo

echo "1. Initializing connection..."
INIT_REQUEST='{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "test-client",
      "version": "1.0.0"
    }
  }
}'

RESPONSE=$(send_request "$INIT_REQUEST")
echo "Response: $RESPONSE"
echo

echo "2. Listing available tools..."
LIST_TOOLS_REQUEST='{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/list",
  "params": {}
}'

RESPONSE=$(send_request "$LIST_TOOLS_REQUEST")
echo "Response: $RESPONSE"
echo

echo "3. Testing fix_parens tool with malformed code..."
TEST_CODE='(defn foo [x\n  (+ x 1'
FIX_PARENS_REQUEST=$(cat <<EOF
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "fix_parens",
    "arguments": {
      "code": "$TEST_CODE"
    }
  }
}
EOF
)

RESPONSE=$(send_request "$FIX_PARENS_REQUEST")
echo "Response: $RESPONSE"
echo

echo "=== Test Complete ==="
