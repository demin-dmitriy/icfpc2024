#!/usr/bin/env python3

from argparse import ArgumentParser

alphabet = 'abcdefghijklmnopqrstuvwxyz'

def process(s :str):
    result = []

    lines = s.splitlines()
    assert lines[0].startswith('solve 3d') or lines[0].startswith('test 3d')
    result.append(lines[0])
    lines = lines[1:]

    v_init = {}
    vs = {}

    i = 0
    for l in lines:
        if l.startswith('init'):
            command, var, value = l.split()
            assert command == 'init'
            assert var not in v_init, "cannot have two init for the same variable"
            assert var in alphabet, "variable names must be in the alphabet"
            v_init[var] = value
            continue

        tokens = l.split()
        if not tokens: continue
        for j, c in enumerate(tokens):
            if c in alphabet and not (j >= 2 and tokens[j-1] == '@' and tokens[j-2] == '?'):
                assert c not in vs, f'Variable {c} is used twice: at {vs[c]} and at {(i, j)}.'
                vs[c] = (i, j)
                # print(c, i, j)

        i += 1

    i = 0
    for l in lines:
        if l.startswith('init'): continue

        j = 0

        last_q = None
        seen_a = False

        list_l = list(l)

        for k, c in enumerate(list_l):
            if c == ' ': continue
            if c == '?':
                last_q = k
            elif c == '@':
                seen_a = True
            elif seen_a and c in alphabet:
                assert c in vs, f'Variable {c} ({i}:{k}) doesn\'t exist'
                target_i, target_j = vs[c]
                list_l[k] = str(i - target_i)
                list_l[last_q] = str((j-1) - target_j)
                seen_a = False
            elif c in alphabet:
                if c in v_init:
                    list_l[k] = v_init[c]
                else:
                    list_l[k] = '.'
            else:
                seen_a = False

            j += 1

        result.append(''.join(list_l))
        if l.replace(' ', '') != '':
            i += 1

    return '\n'.join(result)


def main():
    parser = ArgumentParser('3d_preprocess')
    parser.add_argument('file')
    args = parser.parse_args()
    print(process(open(args.file).read().strip()))


if __name__ == '__main__':
    main()
