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
	cargo doc --no-deps --keep-going
