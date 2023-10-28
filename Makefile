# Targets
.PHONY: all build test format lint clean run setup insert query update delete

all: build test format lint clean run setup insert query update delete

build:
	cd rust_sqlite_project && cargo build

#test:
	#cd rust_sqlite_project && cargo test --quiet

format:
	cd rust_sqlite_project && cargo fmt

lint:
	cd rust_sqlite_project && cargo clippy --quiet

clean:
	cd rust_sqlite_project && cargo clean

run:
	cd rust_sqlite_project && cargo run

setup:
	cd rust_sqlite_project && cargo run setup

insert:
	cd rust_sqlite_project && cargo run insert "Test Artist" 10 5 120

query:
	cd rust_sqlite_project && cargo run query

update:
	cd rust_sqlite_project && cargo run update "Test Artist" 130

delete:
	cd rust_sqlite_project && cargo run delete "Test Artist"
