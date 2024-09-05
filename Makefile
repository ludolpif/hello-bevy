.PHONY: all build-dep build build-release run doc

all: run

build-dep:
	./build-dep.sh

build:
	cargo build

run:
	cargo run

build-release:
	cargo build --no-default-features --release

run-release:
	cargo run --no-default-features --release

doc:
	echo "Warning: rustdoc in 2024 may hang indefinitely on wgpu-doc"
	echo "see: https://github.com/rust-lang/rust/issues/114891"
	cargo doc
