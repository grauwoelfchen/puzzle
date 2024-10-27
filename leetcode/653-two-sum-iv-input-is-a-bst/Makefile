check:
	@cargo check
.PHONY: check

format:
	@cargo fmt --check
.PHONY: format

lint:
	@cargo clippy
.PHONY: lint

fmt: format
.PHONY: fmt

test:
	@cargo test
.PHONY: test

build:
	@cargo build
.PHONY: build

clean:
	@cargo clean
.PHONY: clean

.DEFAULT_GOAL = build
