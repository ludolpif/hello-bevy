#!/bin/sh -xe

# from: https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md
case "$(lsb_release -sc)" in
trixie)
	# rust
	apt-get install rustup clang lld
	# crate: bevy
	apt-get install pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
	apt-get install libwayland-dev libxkbcommon-dev
	# crate: pango-sys
	apt-get install libpango1.0-dev
	# crate: gstreamer-rs
	apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav libgstrtspserver-1.0-dev libges-1.0-dev
	;;
*)
	echo "Untested distro yet, please help us !" >&2
	exit 1
	;;
esac
