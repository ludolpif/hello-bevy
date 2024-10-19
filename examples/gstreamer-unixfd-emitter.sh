#!/bin/bash
GST_DEBUG=3 gst-launch-1.0 videotestsrc ! video/x-raw, width=1920, height=1080, framerate=30/1 ! unixfdsink socket-path=/tmp/lbs0
