#!/bin/sh
#
# Use this script to run your program LOCALLY.
#
#

set -e # Exit early if any commands fail

(
  cd "$(dirname "$0")" # Ensure compile steps are run within the repository directory
  cargo build --release --target-dir=/tmp/shit-shell --manifest-path Cargo.toml
)

#
# - Edit this to change how your program runs locally
exec /tmp/shit-shell/release/shit-shell "$@"