
.PHONY: all build coverage unit-test integration-test test cloc clean

all: build

clean:
	cargo clean

build:
	cargo build

coverage:
	./scripts/gen-coverage.sh

unit-test:
	cargo test

integration-test: build
	./integration_tests.sh --no-kcov

test: unit-test integration-test

cloc:
	cloc src/ test/integration_tests integration_tests.sh Makefile .travis.yml scripts/
