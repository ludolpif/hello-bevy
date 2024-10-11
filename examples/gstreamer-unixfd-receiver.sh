#!/bin/bash
GST_DEBUG=3 gst-launch-1.0 unixfdsrc socket-path=/tmp/lbs0 ! queue ! autovideosink
