
.PHONY: all build coverage unit-test integration-test test cloc clean release presentation

all: build

clean:
	cargo clean

build:
	cargo build

release:
	cargo build --release

coverage:
	./scripts/gen-coverage.sh

unit-test:
	cargo test

integration-test: build
	./integration_tests.sh --no-kcov

test: unit-test integration-test

cloc:
	cloc src/ test/integration_tests integration_tests.sh Makefile .travis.yml scripts/

man2txt: ./man/dotfiles-manager.1
	man ./man/dotfiles-manager.1 | col -b > ./man/dotfiles-manager.1.txt
