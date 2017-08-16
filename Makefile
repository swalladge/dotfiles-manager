
all: build

# install-test:
# 	bundle install --path vendor/bundle

build:
	cargo build

coverage:
	./scripts/gen-coverage.sh

test:
	cargo test
