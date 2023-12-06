#!/bin/sh
#
# Adapted from Deno

set -e

if [ "$OS" = "Windows_NT" ]; then
  if ! command -v unzip >/dev/null; then
    echo "Error: unzip is required to install Swarmd for windows." 1>&2
    exit 1
  fi
fi

# TODO: We should implement a way to differentiate  between windws x86 and
# windows arm.
if [ "$OS" = "Windows_NT" ]; then
  target="x86_64-pc-windows-msvc"
  ending="zip"
else
  ending="tar.gz"
  case $(uname -sm) in
    "Darwin x86_64") target="x86_64-apple-darwin" ;;
    "Darwin arm64") target="aarch64-apple-darwin" ;;
    "Linux aarch64")
      # TODO: We'll just disable dev feature with a feature flag and make it
      # available.
      echo "Error: Official Swarmd builds for Linux aarch64 are not available due to an issue with Deno. (see: https://github.com/denoland/deno/issues/1846 )" 1>&2
      exit 1
      ;;
    *) target="x86_64-unknown-linux-gnu" ;;
  esac
fi

if [ $# -eq 0 ]; then
  swarmd_uri="https://github.com/swarmd-io/swarmd/releases/latest/download/swarmd-${target}.${ending}"
else
  swarmd_uri="https://github.com/swarmd-io/swarmd/releases/download/${1}/swarmd-${target}.${ending}"
fi

swarmd_install="${SWARMD_INSTALL:-$HOME/.swarmd}"
bin_dir="$swarmd_install/bin"
exe="$bin_dir/swarmd"

if [ ! -d "$bin_dir" ]; then
  mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe.$ending" "$swarmd_uri"
if [ "$OS" = "Windows_NT" ]; then
  unzip -d "$bin_dir" -o "$exe.$ending"
else
  tar -xf "$exe.$ending" --strip-components=1 --directory "$bin_dir"
fi

chmod +x "$exe"
rm "$exe.$ending"

echo "Swarmd was installed successfully to $exe"
echo
echo
if command -v swarmd >/dev/null; then
  echo "Run 'swarmd --help' to get started"
else
  case $SHELL in
    /bin/zsh) shell_profile=".zshrc" ;;
    *) shell_profile=".bashrc" ;;
  esac
  echo "Manually add the directory to your \$HOME/$shell_profile (or similar)"
  echo "  export SWARMD_INSTALL=\"$swarmd_install\""
  echo "  export PATH=\"\$SWARMD_INSTALL/bin:\$PATH\""
  echo "Run '$exe --help' to get started"
fi
echo
echo
echo
echo
echo "Join our Discord!! https://discord.gg/QpHDyE3WnW"
