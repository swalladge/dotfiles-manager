#!/bin/bash

# generate local code coverage reports
# depends on kcov

set -e

# so unused/untested code isn't hidden
export RUSTFLAGS="-C link-dead-code"

cargo clean
cargo test --no-run

for file in target/debug/dotfiles_manager-*[^\.d]; do
     mkdir -p "target/cov/$(basename $file)"
     kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done

kcov --merge target/cov target/cov/*

echo "merged coverage report saved to: target/cov/index.html"
