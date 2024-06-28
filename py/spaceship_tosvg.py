#!/usr/bin/env python3

from pathlib import Path
from math import ceil
import json
import sys

        # width="1000" height="1000"

header = lambda min_x, min_y, max_x, max_y, padding: f'''
    <svg version="1.1"
        viewBox="{min_x - padding}, {min_y - padding}, {max_x - min_x + 2 * padding}, {max_y - min_y + 2* padding}"
        xmlns="http://www.w3.org/2000/svg">

        <rect x="{min_x - padding}" y="{min_y - padding}" width="100%" height="100%" style="fill:white; stroke-width:0.1; stroke: black" />
'''

footer = '''
    </svg>
'''

circle = lambda x, y, r=1, color='red', stroke='none': f'<circle cx="{x}" cy="{y}" r="{r}" fill="{color}" stroke="{stroke}" stroke-width="0.2"/>'
text = lambda x, y, t, color='black': f'<text x="{x}" y="{y}" font-size="5" text-anchor="middle" fill="{color}">{t}</text>'

def to_svg(points):
    r = []
    max_x = max(p[0] for p in points + [(0, 0)])
    max_y = max(p[1] for p in points + [(0, 0)])
    min_x = min(p[0] for p in points + [(0, 0)])
    min_y = min(p[1] for p in points + [(0, 0)])

    radius = max(0.2, max(max_x - min_x, max_y - min_y) / 1000)
    padding = 2 * radius
    r.append(header(min_x, min_y, max_x, max_y, padding))

    r.append(circle(0, 0, color='red', r=radius*1.5))
    for (x, y) in points:
        r.append(circle(x, y, color='blue', r=radius))

    r.append(footer)
    return '\n'.join(r)

def parse_point(s):
    x, y = s.split(' ')
    return (int(x), int(y))

def main():
    for p in sys.argv[1:]:
        points = [  parse_point(l) for l in open(p).read().splitlines()]
        svg = to_svg(points)
        # print(svg)
        Path(p).with_suffix('.svg').open('w').write(svg)


if __name__ == '__main__':
    main()
