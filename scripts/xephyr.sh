#!/usr/bin/env bash
# Run case-wm in an embedded X session, for development and testing.
#
# NOTE: If you are developing on a wayland machine, this script will
# not work correctly.
#
# This is intended to be run via the `run-embeded` makefile target
# which will also handle compilation of the examples themselves.
# You will need to have the xephyr utility installed on your system
# for this script to run:
#   https://wiki.archlinux.org/title/Xephyr
CUR_DIR="$(dirname "$(readlink -f "$0")")"
SCREEN_SIZE='1200x600'
XDISPLAY=${XDISPLAY:-:1}
APP="$CUR_DIR/../target/debug/casewm"

touch "$CUR_DIR"/../xephyr.log

Xephyr +extension RANDR -screen ${SCREEN_SIZE} ${XDISPLAY} -ac &
XEPHYR_PID=$!

sleep 1
env DISPLAY="${XDISPLAY}" "$APP" "$CUR_DIR"/../xephyr.log 2>&1 &
WM_PID=$!

trap "kill $XEPHYR_PID && kill $WM_PID && rm $CUR_DIR/../xephyr.log" SIGINT SIGTERM exit

env DISPLAY=${XDISPLAY} $APP &

tail -f "$CUR_DIR/"../xephyr.log

wait $WM_PID
kill $XEPHYR_PID
