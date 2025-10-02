#!/usr/bin/env python3

import subprocess
import json
import sys

SERVER = "../target/debug/parinfer-mcp-server"

def test_server():
    print("=== Testing Parinfer MCP Server ===\n")
    
    # Start the server
    proc = subprocess.Popen(
        [SERVER],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )
    
    def send_request(request, expect_response=True):
        json_str = json.dumps(request)
        print(f"Sending: {json_str}")
        proc.stdin.write(json_str + "\n")
        proc.stdin.flush()
        
        if expect_response:
            response = proc.stdout.readline()
            if not response:
                print("ERROR: No response received", file=sys.stderr)
                sys.exit(1)
            
            print(f"Response: {response}")
            
            try:
                parsed = json.loads(response)
            except json.JSONDecodeError as e:
                print(f"ERROR: Invalid JSON response: {e}", file=sys.stderr)
                sys.exit(1)
            
            # Check if it's a valid JSON-RPC response
            if "jsonrpc" not in parsed or parsed["jsonrpc"] != "2.0":
                print(f"ERROR: Invalid JSON-RPC response (missing or wrong jsonrpc field)", file=sys.stderr)
                sys.exit(1)
            
            # Must have either 'result' or 'error'
            if "result" not in parsed and "error" not in parsed:
                print(f"ERROR: JSON-RPC response missing both 'result' and 'error' fields", file=sys.stderr)
                sys.exit(1)
            
            return parsed
        return None
    
    try:
        # 1. Initialize
        print("1. Initializing connection...")
        init_response = send_request({
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
        })
        print()
        
        # 2. Send initialized notification
        print("2. Sending initialized notification...")
        send_request({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }, expect_response=False)
        print()
        
        # 3. List tools
        print("3. Listing available tools...")
        tools_response = send_request({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        })
        print()
        
        # 4. Test fix_parens
        print("4. Testing fix_parens tool with malformed code...")
        test_code = "(defn foo [x\n  (+ x 1"
        fix_response = send_request({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "fix_parens",
                "arguments": {
                    "code": test_code
                }
            }
        })
        print()
        
        print("=== Test Complete ===")
        print("\n✓ All smoke tests passed")
        
    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        sys.exit(1)
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=2)
        except subprocess.TimeoutExpired:
            proc.kill()

if __name__ == "__main__":
    test_server()
