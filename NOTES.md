# Notes

# Project very start

Read: https://www.rust-lang.org/learn/get-started

```sh
apt install git cargo
git init hello-bevy
cargo init hello-bevy
cd hello-bevy
cat src/main.rs
cat Cargo.toml

echo /target > .gitignore

editor build-dep.sh

editor Makefile # optionnal, small hint for non-rust developers
sudo make build-dep-debian13

cargo add bevy

# for developing, not releasing
cargo add bevy -F dynamic_linking

# from: https://bevyengine.org/learn/quick-start/getting-started/setup/#compile-with-performance-optimizations
editor Cargo.toml

# from: https://bevyengine.org/learn/quick-start/getting-started/setup/#alternative-linkers
mkdir .cargo
editor .cargo/config.toml

# from: https://bevyengine.org/learn/quick-start/getting-started/apps/
editor src/main.rs

make

editor README.md
editor NOTES.md

git add .
git status
git commit -m "project initialization"

dpkg-query -f='${binary:Package;-32} ${Version}\n' -W '*' | grep -E '^(cargo|clang|lld|pkg-config|libx11-dev|libasound2-dev|libudev-dev|libxkbcommon-x11-0|libwayland-dev|libxkbcommon-dev|rustc)[: ]'
#cargo                            1.79.0+dfsg1-2
#clang                            1:16.0-58.1
#libasound2-dev:amd64             1.2.12-1
#libudev-dev:amd64                256.5-1
#libwayland-dev:amd64             1.23.0-1
#libx11-dev:amd64                 2:1.8.7-1+b1
#libxkbcommon-dev:amd64           1.6.0-1+b1
#libxkbcommon-x11-0:amd64         1.6.0-1+b1
#lld:amd64                        1:16.0-58.1
#pkg-config:amd64                 1.8.1-3
#rustc                            1.79.0+dfsg1-2
```
