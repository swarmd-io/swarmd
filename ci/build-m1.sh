#!/bin/bash

# From ripgrep: https://github.com/BurntSushi/ripgrep/blob/84d65865e6febc784b8b0296dd4681d761fa5a67/ci/build-and-publish-m2#L3
# This script builds a swarmd release for the aarch64-apple-darwin target.
#
# Once GitHub Actions has proper support for Apple silicon, we should add it
# to our release workflow and drop this script.

set -e

version="$1"
if [ -z "$version" ]; then
  echo "missing version" >&2
  echo "Usage: "$(basename "$0")" <version>" >&2
  exit 1
fi

target=aarch64-apple-darwin
OPENSSL_STATIC=1 cargo build --bin swarmd --release --target $target
BIN=target/$target/release/swarmd
NAME=swarmd-$target
ARCHIVE="deployment/m1/$NAME"

mkdir -p "$ARCHIVE"/doc
cp "$BIN" "$ARCHIVE"/
strip "$ARCHIVE/swarmd"
cp cli/README.md "$ARCHIVE"/
cp cli/CHANGELOG.md "$ARCHIVE"/doc/
# "$BIN" --generate complete-bash > "$ARCHIVE/complete/rg.bash"
# "$BIN" --generate complete-fish > "$ARCHIVE/complete/rg.fish"
# "$BIN" --generate complete-powershell > "$ARCHIVE/complete/_rg.ps1"
# "$BIN" --generate complete-zsh > "$ARCHIVE/complete/_rg"
# "$BIN" --generate man > "$ARCHIVE/doc/rg.1"

# tar czf "$ARCHIVE.tar.gz" "$NAME"
tar czvf "$ARCHIVE.tar.gz" -C deployment/m1 "$NAME"
shasum -a 256 "$ARCHIVE.tar.gz" > "$ARCHIVE.tar.gz.sha256"
gh release upload "$version" "$ARCHIVE.tar.gz" "$ARCHIVE.tar.gz.sha256"
