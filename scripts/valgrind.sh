#!/usr/bin/env bash
set -euo pipefail

docker build -t rust-valgrind-tests .
docker run --rm -v $(pwd):/app rust-valgrind-tests
