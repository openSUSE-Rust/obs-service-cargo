#!/bin/bash

set -euo pipefail

SCRIPTPATH="$( cd "$(dirname "$0")" || exit ; pwd -P )"

echo "# Downloading bonk"
curl -LJ0 "https://github.com/elliot40404/bonk/archive/refs/tags/v0.3.2.tar.gz" --output /tmp/bonk-0.3.2.tar.gz
echo "# Downloading s390-tools"
curl -LJ0 "https://github.com/ibm-s390-linux/s390-tools/archive/refs/tags/v2.29.0.tar.gz" --output /tmp/s390-tools-2.29.0.tar.gz
echo "# Downloading flux"
curl -LJ0 "https://github.com/influxdata/flux/archive/refs/tags/v0.194.4.tar.gz" --output /tmp/flux-0.194.4.tar.gz

echo "# Downloading VBox-Starter"

curl -LJ0 "https://gitlab.com/catsfood/VBox-Starter/-/archive/v3.1.0/VBox-Starter-v3.1.0.tar.gz" --output /tmp/VBox-Starter-v3.1.0.tar.gz

echo "# Generating tarball"
"${SCRIPTPATH}"/target/release/cargo_vendor --src /tmp/bonk-0.3.2.tar.gz --outdir /tmp
echo "# No tarball to remove"

echo "# Generating tarball"
"${SCRIPTPATH}"/target/release/cargo_vendor --src /tmp/s390-tools-2.29.0.tar.gz --outdir /tmp --cargotoml rust/pvsecret/Cargo.toml --cargotoml rust/utils/Cargo.toml --cargotoml rust/pv/Cargo.toml
echo "# Removing vendored tarball"
rm /tmp/vendor.tar.zst

echo "# Generating tarball"
"${SCRIPTPATH}"/target/release/cargo_vendor --src /tmp/flux-0.194.4.tar.gz --outdir /tmp --cargotoml libflux/Cargo.toml
echo "# Removing vendored tarball"
rm /tmp/vendor.tar.zst

echo "# Generating tarball"
"${SCRIPTPATH}"/target/release/cargo_vendor --src /tmp/VBox-Starter-v3.1.0.tar.gz --outdir /tmp
echo "# Removing vendored tarball"
rm /tmp/vendor.tar.zst

echo "# Test tagging"
"${SCRIPTPATH}"/target/release/cargo_vendor --src /tmp/s390-tools-2.29.0.tar.gz --outdir /tmp --cargotoml rust/pvsecret/Cargo.toml --cargotoml rust/utils/Cargo.toml --cargotoml rust/pv/Cargo.toml --tag "rust-component"

if [ ! -f "/tmp/vendor-rust-component.tar.zst" ]
then
    # Fail
    exit 1
fi

