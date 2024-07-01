#!/usr/bin/env python3
from dataclasses import dataclass

@dataclass
class Op:
    op: str
    src1: str
    src2: str
    dst: str


block1 = lambda op1: (
f'''
. {op1.src2} .
{op1.src1} {op1.op} .
. . .
? @ {op1.dst}
. 1 .
''')


block2 = lambda op1, op2: (
f'''
. {op1.src2} .  . {op2.src2} .
{op1.src1} {op1.op} .  {op2.src1} {op2.op} .
. . .  . . .
? @ {op1.dst}  ? @ {op2.dst}
. 1 .  . 1 .
''')


class State:
    def __init__(self, problem_n):
        self.problem_n = problem_n
        self.inits = []
        self.lines = []
        self.blocks = []
        self.ops = []

    def init(self, var, value):
        self.inits.append(f'init {var} {value}')

    def op(self, op, arg1, arg2, *, to):
        self.ops.append(Op(op, arg1, arg2, to))

    def copy2(self, *, src, to1, to2):
        pass

    def block(self, b):
        self.blocks.append(b)

    def to_str(self):
        r = []
        r.append(f'solve 3d{self.problem_n}')
        for init in self.inits:
            r.append(init)

        for op1, op2 in zip(self.ops[::2], self.ops[1::2]):
            r.append(block2(op1, op2))
        if len(self.ops) % 2 == 1:
            r.append(block1(self.ops[-1]))

        for b in self.blocks:
            r.append(b)

        return '\n'.join(r)


s = State('7')

s.init('a', 'A')
s.init('b', 'A')
s.init('c', 0)

s.op('/', 'd', 10, to='h')
s.op('%', 'b', 10, to='e')
s.op('*', 'c', 10, to='f')
s.op('%', 'f', 'e', to='g')

# s.copy2(src='h', to1='a', to2='b')

s.block(
'''
    . . . . 0
    . . < a =
    ? @ d . S
    . 1 . . .
'''
)

s.block(
'''
    . . < h .
    ? @ a v .
    . 1 . . .
    . . ? @ b
    . . . 1 .
'''
)

print(s.to_str())

def main():
    pass


if __name__ == '__main__':
    main
