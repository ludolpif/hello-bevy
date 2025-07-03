.PHONY: all build-dep build-dev run-dev dump-schedule-update-dev strace build-release run-release dump-schedule-update-release build-release-with-debug run-release-with-debug doc

all: run-dev

build-dep:
	sudo ./build-dep-system.sh
	./build-dep-user.sh

build-dev:
	cargo build

run-dev:
	cargo run

dump-schedule-update-dev:
	cargo run -- dump-schedule update

strace:
	scripts/strace.sh

build-release:
	cargo build --no-default-features --release

run-release:
	cargo run --no-default-features --release

dump-schedule-update-release:
	cargo run --no-default-features --release -- dump-schedule update

build-release-with-debug:
	cargo build --no-default-features --profile=release-with-debug

run-release-with-debug:
	cargo run --no-default-features --profile=release-with-debug

doc:
	cargo doc --no-deps --keep-going
