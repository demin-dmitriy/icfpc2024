#!/usr/bin/env python3

import math
from pathlib import Path
from argparse import ArgumentParser

from tsp import solve_tsp


def dist(a, b):
    return int(math.sqrt(pow(b[0]-a[0], 2) + pow(b[1]-a[1], 2)))

def is_reachable_1d(current_pos, current_speed, target, steps):
    center = current_pos + current_speed * steps
    radius = (steps * (steps + 1)) / 2

    return (target >= center - radius) and (target <= center + radius)

def is_reachable(current_pos, current_speed, target, steps):
    return is_reachable_1d(current_pos[0], current_speed[0], target[0], steps) and is_reachable_1d(current_pos[1], current_speed[1], target[1], steps)

def distspeed(a, b):
    steps = 1;
    current_speed = (0, 0)
    while True:
        target_reachable = is_reachable(a, current_speed, b, steps);
        if target_reachable: 
            return steps
        steps += 1;

def main():
    parser = ArgumentParser('com', description="Example usage: py/com.py 'get index'")
    parser.add_argument('-i', '--input', help = 'Path to file with spaceship task')
    parser.add_argument('-o', '--output', help = 'Path to result file with reordered spaceship task')
    args = parser.parse_args()

    task_path = Path(args.input)
    task = task_path.read_text().rstrip()
    lines = task.split('\n')
    points = []
    points.append((0, 0))
    for line in lines:
        x, y = line.split(' ')
        points.append((int(x), int(y)))

    dist_matrix = []
    for point_a in points:
        row = []
        for point_b in points:
            row.append(distspeed(point_a, point_b))
        dist_matrix.append(row)

    route, cost = solve_tsp(dist_matrix)
    print(cost)

    result = ''
    route.pop(0)
    for ind in route:
        point = points[ind]
        result += f'{point[0]} {point[1]}\n'

    new_task_path = Path(args.output)
    new_task_path.write_text(result)


if __name__ == '__main__':
    main()
