#!/usr/bin/env bash

set -euo pipefail

for i in $(seq -99 0); do
    echo x=$i
    cat ./py/solutions/3d/3.solution | sed "s/x/$i/" |  py/com.py --stdin
    sleep 3
done
