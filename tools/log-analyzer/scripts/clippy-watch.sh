#!/bin/sh

while true; do
  reset
  cargo clippy
  inotifywait -e modify,create src
done
