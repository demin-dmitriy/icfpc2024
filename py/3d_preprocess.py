#!/usr/bin/env python3

from argparse import ArgumentParser

alphabet = 'abcdefghijklmnopqrstuwxyzCDEFGHIJKLMNOPQRTUVWXYZ'

def process(s :str):
    result = []

    lines = s.splitlines()
    assert lines[0].startswith('solve 3d') or lines[0].startswith('test 3d'), lines[0]
    result.append(lines[0])
    lines = lines[1:]

    v_init = {}
    is_init_only = set()
    is_multiple_read_locs = set()
    vs = {}

    i = 0
    for l in lines:
        if l.startswith('init'):
            command, var, value = l.split()
            assert command == 'init' or command == 'init_only'
            assert var not in v_init, "cannot have two init for the same variable"
            assert var in alphabet, "variable names must be in the alphabet"
            v_init[var] = value
            if command == 'init_only':
                is_init_only.add(var)
            continue
        elif l.startswith('multiple_read_locs'):
            command, var = l.split()
            assert command == 'multiple_read_locs'
            assert var in alphabet, "variable names must be in the alphabet"
            is_multiple_read_locs.add(var)
            continue


        tokens = l.split()
        if not tokens: continue
        for j, c in enumerate(tokens):
            if c in alphabet and not (j >= 2 and tokens[j-1] == '@' and tokens[j-2] == '?'):
                if c not in is_init_only:
                    assert c not in vs, f'Variable {c} is used twice: at {vs[c]} and at {(i, j)}.'
                    vs[c] = (i, j)
                # print(c, i, j)

        i += 1

    i = 0
    for l in lines:
        if l.startswith('init') or l.startswith('multiple_read_locs'): continue

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
                last_q = None
                seen_a = False
            elif c in alphabet:
                if c in v_init:
                    list_l[k] = v_init[c]
                else:
                    list_l[k] = '.'
            else:
                assert c in ['.', 'A', 'B', 'S', 'v', '<', '>', '^', '+', '-', '*', '/', '%', '=', '#'] or c.isdigit(), f'Invalid c={c}'
                if last_q is not None:
                    print(f'WARNING: unused ? at {i}{last_q}')
                    last_q = None
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
