#!/bin/bash
# Quick an dirty only for linux with rustup stable-x86_64-unknown-linux-gnu toolchain set as default
b=$(pwd)
p=$(basename $(pwd))
CARGO=~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo CARGO_MANIFEST_DIR=$b CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=$p CARGO_PKG_README=README.md CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.1.0 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=1 CARGO_PKG_VERSION_PATCH=0 CARGO_PKG_VERSION_PRE='' LD_LIBRARY_PATH=$(
	echo $b/target/debug{/build/blake3-*/out,/deps,} ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib{/rustlib/x86_64-unknown-linux-gnu/lib,} | tr ' ' ':'
) strace -ffo /dev/shm/hello-bevy target/debug/hello-bevy
