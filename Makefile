.PHONY: help clean test run

.DEFAULT: help

help:
	@echo ""
	@echo "Options:"
	@echo "\tmake clean 				  - remove artifacts"
	@echo "\tmake build 				  - build release artifact"
	@echo "\tmake fmt 				    - format files"
	@echo "\tmake vet 				    - lint and validate files"
	@echo "\tmake test  				  - run tests"
	@echo "\tmake test  				  - update dependencies"
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

clean-build: clean build

clean-test: clean test

update:
	cargo update
