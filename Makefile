# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	#cargo fmt --quiet
	cd rust_sqlite_project && cargo fmt --quiet


# Run clippy for linting
lint:
	#cargo clippy --quiet
	cd rust_sqlite_project && cargo clippy --quiet

# Run tests
test:
	#cargo test --quiet
	cd rust_sqlite_project && cargo test --quiet
# Build and run the project
run:
	#cargo run
	cd rust_sqlite_project && cargo run
# Build release version
release:
	#cargo build --release
	cd rust_sqlite_project && cargo build --release
# Install Rust toolchain if needed
install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks

# Create a database entry
create:
	#cargo run insert "Artist Test" 10 5 120
	cd rust_sqlite_project && cargo run insert "Artist Test" 10 5 120
# Read from the database
read:
	#cargo run query
	cd rust_sqlite_project && cargo run query
# Update a database entry
update:
	#cargo run update "Artist Test" 130
	cd rust_sqlite_project && cargo run update "Artist Test" 130
# Delete a database entry
delete:
	#cargo run delete "Artist Test"
	cd rust_sqlite_project && cargo run delete "Artist Test"
# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
