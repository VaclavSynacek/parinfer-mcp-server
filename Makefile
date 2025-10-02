.PHONY: build build-cli build-mcp clean test help

TARGET_DIR = target
CLI_BINARY = parinfer-cli
MCP_BINARY = parinfer-mcp-server
RELEASE_CLI = $(TARGET_DIR)/release/$(CLI_BINARY)
RELEASE_MCP = $(TARGET_DIR)/release/$(MCP_BINARY)

help:
	@echo "Available targets:"
	@echo "  build      - Build both CLI tool and MCP server (release)"
	@echo "  build-cli  - Build only the CLI tool (release)"
	@echo "  build-mcp  - Build only the MCP server (release)"
	@echo "  test       - Run all tests"
	@echo "  clean      - Remove build artifacts"
	@echo "  help       - Show this help message"

build: build-cli build-mcp

build-cli:
	cargo build --release --bin $(CLI_BINARY)
	@echo "CLI binary built at: $(RELEASE_CLI)"

build-mcp:
	cargo build --release --bin $(MCP_BINARY)
	@echo "MCP server binary built at: $(RELEASE_MCP)"

test:
	cargo test

clean:
	cargo clean
	@echo "Build artifacts cleaned"
