#!/usr/bin/env bash

set -euo pipefail

for i in $(seq 1 25); do
    echo processing $i
    ./spaceship_tosvg.py history/spaceship/$i
    inkscape -w 1024 history/spaceship/$i.svg -o history/spaceship/$i.png
done
