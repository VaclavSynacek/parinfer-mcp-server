.PHONY: build test clean help

TARGET_DIR = target
MCP_BINARY = parinfer-mcp-server
RELEASE_MCP = $(TARGET_DIR)/release/$(MCP_BINARY)

help:
	@echo "Available targets:"
	@echo "  build      - Build the MCP server (release)"
	@echo "  test       - Run all tests"
	@echo "  clean      - Remove build artifacts"
	@echo "  help       - Show this help message"

build:
	cargo build --release --bin $(MCP_BINARY)
	@echo "MCP server binary built at: $(RELEASE_MCP)"

test:
	cargo test

clean:
	cargo clean
	@echo "Build artifacts cleaned"
