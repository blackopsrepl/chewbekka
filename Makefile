SHELL := /bin/bash
.PHONY: help config rust-version install build format lint test alpha beta minor release

help:
	@echo "Makefile Commands:"
	@echo "  config               - Set up the Rust environment."
	@echo "  install              - Install chewbekka in /usr/local/bin."
	@echo "  build                - Build chewbekka with cargo"
	@echo "  format               - Format source code with cargo fmt"
	@echo "  lint                 - Lint source code with cargo clippy"
	@echo "  test                 - Test chewbekka with cargo test"
	@echo "  alpha                - Generate changelog and create an alpha tag."
	@echo "  beta                 - Generate changelog and create an beta tag."
	@echo "  minor                - Generate changelog and create an minor tag."
	@echo "  release              - Generate changelog and create a release tag."

all: format lint build test

config:
	@echo "Updating rust toolchain"
	rustup update stable
	rustup default stable

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

install:
	@echo "Configuring ollama model"
	ollama create chewbekka -f conf/chewbekka.prompt
	@echo "Building all projects with cargo, in release mode"
	./util/build-release.sh
	@echo "Copying chewbekka binary to /usr/local/bin"
	sudo cp target/release/chewbekka /usr/local/bin/

build:
	@echo "Building all projects with cargo"
	./util/build.sh

format:
	@echo "Formatting all projects with cargo"
	./util/format.sh

lint:
	@echo "Linting all projects with cargo"
	./util/lint.sh

test:
	@echo "Testing all projects with cargo"
	./util/test.sh

alpha:
	@echo "Generating changelog and tag"
	commit-and-tag-version --prerelease alpha

beta:
	@echo "Generating changelog and tag"
	commit-and-tag-version --prerelease beta

minor:
	@echo "Generating changelog and tag"
	commit-and-tag-version --release-as minor

patch:
	@echo "Generating changelog and tag"
	commit-and-tag-version --release-as patch

release:
	@echo "Generating changelog and tag"
	commit-and-tag-version
