.PHONY: build clean test help

TARGET_DIR = target
BINARY_NAME = parinfer-cli
RELEASE_BINARY = $(TARGET_DIR)/release/$(BINARY_NAME)
DEBUG_BINARY = $(TARGET_DIR)/debug/$(BINARY_NAME)

help:
	@echo "Available targets:"
	@echo "  build  - Build the release version of the CLI tool"
	@echo "  test   - Run all tests"
	@echo "  clean  - Remove build artifacts"
	@echo "  help   - Show this help message"

build:
	cargo build --release
	@echo "Binary built at: $(RELEASE_BINARY)"

test:
	cargo test

clean:
	cargo clean
	@echo "Build artifacts cleaned"
