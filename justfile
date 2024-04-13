

# Build the project in release mode
build-release:
    @echo "Building release version"
    cargo build --release

# Run the project
run:
    @echo "Running the project"
    cargo run

# Update dependencies in cargo
update-deps:
    @echo "Updating dependencies"
    cargo update

# check the project for errors without building
check:
    @echo "Checking the project"
    cargo check

# Format the Rust code according to rustfmt standards
fmt:
    @echo "Formatting the code"
    cargo fmt

# Run clippy to lint the project
lint:
    @echo "Linting the project"
    cargo clippy

# Display this help
help:
    @just --list
