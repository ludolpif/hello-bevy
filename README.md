# Purpose
My attempt to build and run a minimal Bevy project.

# Building
## Linux
I currently use Debian trixie (testing) for that. It will generate a binary with low system dependancy but needs `GLIBC2.38` symbols which is not a good choice to distribute it in 2024. This envrionnement is great for now to explore recent things around `rust, `bevy`, `vim`.
```sh
git clone https://github.com/ludolpif/hello-bevy
cd hello-bevy
sudo ./build-dep-system.sh
./build-dep-user.sh
cargo build
cargo run

cat Makefile # act as cheat sheet for non-rust developpers
```
# Contributing
See [DEVELOPPING.md](DEVELOPPING.md)
