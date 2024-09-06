# Purpose
My attempt to build and run a minimal Bevy project.

# Building
## Linux
I currently use Debian trixie (testing) for that. It will generate a binary with low system dependancy but needs GLIBC2.38 which is not a good choice to dsitribute it in 2024. This envrionnement is great for now to explore recent things around `rust, `bevy`, `vim`.
```sh
git clone https://github.com/ludolpif/hello-bevy
cd hello-bevy
sudo ./build-dep-system.sh
./build-dep-user.sh
cargo build
cargo run
```
# How it was made
See [NOTES.md](NOTES.md)
