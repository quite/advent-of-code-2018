#!/bin/sh

for d in day*; do
  (cd "$d" && cargo run -q --release)
done
