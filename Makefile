.PHONY: help clean test run build fmt vet

.DEFAULT: help

help:
	@echo ""
	@echo "Options:"
	@echo "\tmake clean 				  - remove artifacts"
	@echo "\tmake build 				  - build release artifact"
	@echo "\tmake fmt 				    - format files"
	@echo "\tmake vet 				    - lint and validate files"
	@echo "\tmake test  				  - run tests"
	@echo "\tmake bench  				  - run benchmark tests"
	@echo "\tmake update  			  - update dependencies"
	@echo ""

clean:
	cargo clean

fmt:
	cargo fmt

vet:
	cargo clippy

build: fmt vet
	cargo build --release

test: build
	cargo test --verbose

bench: build
	cargo bench --verbose

clean-build: clean build

clean-test: clean test

update:
	cargo update

run:
	cargo run
