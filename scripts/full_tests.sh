#!/usr/bin/env bash
set -euo pipefail

cargo check
cargo test
cargo +nightly miri test

$(dirname "$0")/valgrind.sh
