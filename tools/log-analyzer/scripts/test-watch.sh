#!/bin/sh

while true; do
  reset
  cargo test $1
  inotifywait -r -e modify,create src data
done
