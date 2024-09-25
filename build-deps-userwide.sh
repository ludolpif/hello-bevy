#!/bin/sh -xe

# from: https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md
case "$(lsb_release -sc)" in
trixie)
	rustup default stable
	rustup component add rust-src
	rustup component add rust-analyzer
	;;
*)
	echo "Untested distro yet, please help us !" >&2
	exit 1
	;;
esac
