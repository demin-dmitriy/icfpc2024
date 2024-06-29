#!/usr/bin/env bash

set -euo pipefail

solution_filepath="solutions/spaceship/${1}"
solution=`cat ${solution_filepath}`
submission="solve spaceship${1} ${solution}"
echo $submission | ./com.py --stdin