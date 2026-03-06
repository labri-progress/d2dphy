#!/bin/sh

while true; do
  reset
  cargo run $1
  inotifywait -r -e modify,create src data
done
