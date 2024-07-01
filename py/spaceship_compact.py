#!/usr/bin/env python3

from pathlib import Path
import lex

task_i = 24
s = open(Path(__file__).parent / f'solutions/spaceship/{task_i}').read().strip()
# s='1234567891232134'

assert len(s) % 2 == 0, "doesn't work with even length yet"

r = []
for a, b in zip(s[0::2], s[1::2]):
    assert int(a) in range(1, 10)
    assert int(b) in range(1, 10)
    d = (int(a) - 1) * 9 + (int(b) - 1)
    assert 0 <= d < 94
    r.append(lex.alphabet[d])

r = ''.join(r)

assert len(r) <= 1_000_000

prefix = lex.encode_string(f'solve spacehip{task_i} ')


Y = 'Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx'
if_ = lambda cond, then, else_: f'? {cond} {then} {else_}'
one_index = f'I{lex.encode_int(lex.alphabet.index("1"))}'
int_1 = f'I{lex.encode_int(1)}'
int_9 = f'I{lex.encode_int(9)}'
int_94 = f'I{lex.encode_int(94)}'

decode_one = f'Lc B. U$ B+ {one_index} B/ vc {int_9} U$ B+ {one_index} B% vc {int_9}'
decode = f'Lf Ls {if_("B= vs S", "S", f"B. B$ {decode_one} U# BT {int_1} vs B$ vf BD {int_1} vs")}'
request_str = f'B. S{prefix} B$ B$ {Y} {decode} S{lex.encode_string(r)}'

print(request_str)
