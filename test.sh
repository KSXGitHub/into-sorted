#! /bin/bash
set -o errexit -o nounset -o pipefail
cd "$(dirname "$0")"

run() {
  echo 'exec> cargo' "$@" >&2
  cargo "$@"
}

run clippy --no-default-features -- -D warnings
run clippy --all-features -- -D warnings
run clippy -- -D warnings
run clippy --features alloc -- -D warnings

run test --no-default-features
run test --all-features
run test
run test --features alloc

RUSTFLAGS='-D warnings' run doc
