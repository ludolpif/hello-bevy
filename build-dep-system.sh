#!/bin/sh -xe

# from: https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md
case "$(lsb_release -sc)" in
trixie)
	apt-get install rustup clang lld
	apt-get install pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
	apt-get install libwayland-dev libxkbcommon-dev
	;;
*)
	echo "Untested distro yet, please help us !" >&2
	exit 1
	;;
esac
