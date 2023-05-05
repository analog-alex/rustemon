PROJECT_NAME = rustemon
BIN_NAME = $(PROJECT_NAME)
SRC_DIR = src
BUILD_DIR = build

# Declare phony targets to avoid file/directory name conflicts
.PHONY: build clean build_dev run install_to_path

# Build the release version of the project
build:
	cargo build --release --locked

# Clean up build artifacts
clean:
	cargo clean

# Build the development version of the project
build_dev:
	cargo build

# Run the development version of the project
run: build_dev
	cargo run

check_path:
	@echo "Checking if /usr/local/bin is in the PATH..."
	@echo $$PATH | grep -q "/usr/local/bin" || (echo "/usr/local/bin not found in PATH. Please add it." && false)

# Install the release binary to the specified path
install_to_path: check_path build
	sudo cp target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)
