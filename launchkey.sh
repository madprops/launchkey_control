#!/usr/bin/env bash
# Detects reconnection and restarts the program

port=$(amidi --list-devices | grep "Launchkey MK2 25 Launchkey InCo" | sed -n '/^IO/s/.*\(hw[^ ]*\).*/\1/p' | tr -d '\n')
disconnected=false
launchkey_control &

while true; do
  if amidi -p $port -S 0; then
    if $disconnected; then
      disconnected=false
      killall launchkey_control
      sleep 3
      launchkey_control &
    fi
  else
    disconnected=true
  fi

  sleep 10
done