# Notes

## Project very start

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

make # shows an empty window and a console "Hello, World!"

editor README.md
editor NOTES.md

git status
git add .
git commit -m "project initialization"

dpkg-query -f='${binary:Package;-32} ${Version}\n' -W '*' | grep -E '^(clang|lld|pkg-config|libx11-dev|libasound2-dev|libudev-dev|libxkbcommon-x11-0|libwayland-dev|libxkbcommon-dev)[: ]'
#clang                            1:16.0-58.1
#libasound2-dev:amd64             1.2.12-1
#libudev-dev:amd64                256.5-1
#libwayland-dev:amd64             1.23.0-1
#libx11-dev:amd64                 2:1.8.7-1+b1
#libxkbcommon-dev:amd64           1.6.0-1+b1
#libxkbcommon-x11-0:amd64         1.6.0-1+b1
#lld:amd64                        1:16.0-58.1
#pkg-config:amd64                 1.8.1-3

rustup --version                                                                                           |
#rustup 1.26.0 (2024-08-30)
#info: This is the version for the rustup toolchain manager, not the rustc compiler.
#info: The currently active `rustc` version is `rustc 1.81.0 (eeb90cda1 2024-09-04)`

cargo add bevy -F bevy_dev_tools
editor src/playground.rs
editor src/main.rs

make # shows a FPS counter

editor NOTES.md

git status
git add .
git commit -m "split main.rs creating a playground.rs module. add basic FPS overlay"
```

## Develpement environnement configuration

On a debian trixie, make sure that `./build-dep-system` and `./build-dep-user` have been ran. Check `rust-analyser` availability:

```
$ rust-analyzer --version
rust-analyzer 1.81.0 (eeb90cd 2024-09-04)
```

Then install some system-wide vim packages:
```sh
sudo apt install vim vim-ale vim-syntastic
```
Then set current use `~/.vimrc` with:
```vim
runtime! defaults.vim
runtime! debian.vim
if &diff
  syntax off
else
  set mouse=
end
set background=dark

filetype plugin indent on

" vim-ale, usage: https://github.com/dense-analysis/ale?tab=readme-ov-file#usage
packadd! ale
let g:ale_linters = {'rust': ['analyzer']}
let g:ale_virtualtext_cursor = 'current'
let g:ale_set_highlights = 0
let g:ale_sign_column_always = 1
" You should not turn this setting on if you wish to use ALE as a completion
" source for other completion plugins, like Deoplete.
let g:ale_completion_enabled = 1
set omnifunc=ale#completion#OmniFunc
"let g:ale_completion_autoimport = 0

nmap <silent> <C-k> <Plug>(ale_previous_wrap)
nmap <silent> <C-j> <Plug>(ale_next_wrap)
EOT
```

- `vim src/main.rs`
- `:set filetype` should return `=rust`
- `:ALEInfo` should contain : `Enabled Linters: ['analyzer']`
- mangle the end of some struct defined in devmode crate
- press `Ctrl+X Ctrl+O` to force `ale` to generate a first omni-completion
- use `Ctrl+N` to choose a completion proposition (then return)
- now omnicompletion should trigger itself automatically
- code errors are checked only at file save. juste `:w` and see virtual text where the gutter indicate it.
