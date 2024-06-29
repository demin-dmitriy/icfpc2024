#!/usr/bin/env python3

import lex
import numpy as np
from argparse import ArgumentParser
import sys
from typing import Any
from dataclasses import dataclass
sys.setrecursionlimit(15000)


def string_to_int(s):
    sum = 0
    for c in s:
        sum *= 94
        sum += lex.char_to_index[ord(c)]
    return sum


def int_to_string(v):
    r = []
    while v != 0:
        v, ch = divmod(v, 94)
        r.append(lex.alphabet[ch])
    return ''.join(reversed(r))


def get_subtree(tokens):
    balance = 1
    for i, tok in enumerate(tokens):
        balance -= 1
        if isinstance(tok, (lex.Unary, lex.Lambda)):
            balance += 1
        elif isinstance(tok, lex.Binary):
            balance += 2
        elif isinstance(tok, lex.Ternary):
            balance += 3
        elif isinstance(tok, (int, str, bool, lex.Var)):
            pass
        else:
            print(tok, type(tok))
            assert False
        if balance == 0:
            return tokens[:i + 1]

    assert False

def is_y_combinator(tokens):
    print('Candidate:', tokens)

    return False
    pass


@dataclass
class Lazy:
    evaluated_expr: None | Any
    tokens: np.array
    ctx: dict[str, np.array]

    def value(self):
        if self.evaluated_expr is None:
            self.evaluated_expr = eval_tokens(self.tokens, self.ctx)
        return self.evaluated_expr




def eval_head(tokens, ctx: dict[str, np.array]):
    v = eval_tokens(tokens, ctx)
    while isinstance(v, Lazy):
        v = v.value()
    return v


def eval_tokens(tokens, ctx: dict[str, np.array]):
    assert len(get_subtree(tokens)) == len(tokens)

    for i, tok in enumerate(tokens):
        if isinstance(tok, (lex.Unary, lex.Lambda)):
            arg1 = get_subtree(tokens[i + 1:])
        elif isinstance(tok, lex.Binary):
            arg1 = get_subtree(tokens[i + 1:])
            arg2 = get_subtree(tokens[i + len(arg1) + 1:])
        elif isinstance(tok, lex.Ternary):
            arg1 = get_subtree(tokens[i + 1:])
            arg2 = get_subtree(tokens[i + len(arg1) + 1:])
            arg3 = get_subtree(tokens[i + len(arg1) + len(arg2) + 1:])
        elif isinstance(tok, (int, str, bool, lex.Var)):
            pass
        else:
            assert False

        match tok:
            case lex.Binary.Add:
                return eval_head(arg1, ctx) + eval_head(arg2, ctx)
            case lex.Binary.Sub:
                return eval_head(arg1, ctx) - eval_head(arg2, ctx)
            case lex.Binary.Mul:
                return eval_head(arg1, ctx) * eval_head(arg2, ctx)
            case lex.Binary.Div:
                v1 = eval_head(arg1, ctx)
                v2 = eval_head(arg2, ctx)
                s1 = v1 // abs(v1)
                s2 = v2 // abs(v2)
                return s1 * s2 * (abs(v1) // abs(v2))
            case lex.Binary.Mod:
                v1 = eval_head(arg1, ctx)
                v2 = eval_head(arg2, ctx)
                s1 = v1 // abs(v1)
                s2 = v2 // abs(v2)
                return s1 * s2 * (abs(v1) % abs(v2))
            case lex.Binary.Less:
                return eval_head(arg1, ctx) < eval_head(arg2, ctx)
            case lex.Binary.Greater:
                return eval_head(arg1, ctx) > eval_head(arg2, ctx)
            case lex.Binary.Eq:
                return eval_head(arg1, ctx) == eval_head(arg2, ctx)
            case lex.Binary.Or:
                return eval_head(arg1, ctx) or eval_head(arg2, ctx)
            case lex.Binary.And:
                return eval_head(arg1, ctx) and eval_head(arg2, ctx)
            case lex.Binary.Concat:
                return eval_head(arg1, ctx) + eval_head(arg2, ctx)
            case lex.Binary.Take:
                return  eval_head(arg2, ctx)[:eval_head(arg1, ctx)]
            case lex.Binary.Drop:
                return eval_head(arg2, ctx)[eval_head(arg1, ctx):]
            case lex.Binary.Apply:
                return eval_head(arg1, ctx)(Lazy(None, arg2, ctx))
            case lex.Unary.Neg:
                return -eval_head(arg1, ctx)
            case lex.Unary.Not:
                return not eval_head(arg1, ctx)
            case lex.Unary.ToInt:
                return string_to_int(eval_head(arg1, ctx))
            case lex.Unary.ToStr:
                return int_to_string(eval_head(arg1, ctx))

            case lex.Ternary.If:
                return eval_head(arg2, ctx) if eval_head(arg1, ctx) else eval_head(arg3, ctx)

            case _:
                if isinstance(tok, lex.Lambda):
                    return lambda x: eval_tokens(arg1, { **ctx, tok.variable: x })
                elif isinstance(tok, lex.Var):
                    return ctx[tok.variable]
                elif isinstance(tok, (int, str, bool)):
                    return tok

                # print(tok, type(tok))
                assert False, f'Unknown token {tok}'

def paren(tokens):
    s = pprint(tokens)
    if len(tokens) == 1:
        return f'{s}'
    return f'({s})'


def pprint(tokens):
    tokens = np.array(tokens, dtype=np.dtype(object))

    assert len(get_subtree(tokens)) == len(tokens)

    for i, tok in enumerate(tokens):
        if isinstance(tok, (lex.Unary, lex.Lambda)):
            arg1 = get_subtree(tokens[i + 1:])
        elif isinstance(tok, lex.Binary):
            arg1 = get_subtree(tokens[i + 1:])
            arg2 = get_subtree(tokens[i + len(arg1) + 1:])
        elif isinstance(tok, lex.Ternary):
            arg1 = get_subtree(tokens[i + 1:])
            arg2 = get_subtree(tokens[i + len(arg1) + 1:])
            arg3 = get_subtree(tokens[i + len(arg1) + len(arg2) + 1:])
        elif isinstance(tok, (int, str, bool, lex.Var)):
            pass
        else:
            assert False

        match tok:
            case lex.Binary.Add:
                return f'{paren(arg1)} + {paren(arg2)}'
            case lex.Binary.Sub:
                return f'{paren(arg1)} - {paren(arg2)}'
            case lex.Binary.Mul:
                return f'{paren(arg1)} * {paren(arg2)}'
            case lex.Binary.Div:
                return f'{paren(arg1)} / {paren(arg2)}'
            case lex.Binary.Mod:
                return f'{paren(arg1)} % {paren(arg2)}'
            case lex.Binary.Less:
                return f'{paren(arg1)} < {paren(arg2)}'
            case lex.Binary.Greater:
                return f'{paren(arg1)} > {paren(arg2)}'
            case lex.Binary.Eq:
                return f'{paren(arg1)} == {paren(arg2)}'
            case lex.Binary.Or:
                return f'{paren(arg1)} or {paren(arg2)}'
            case lex.Binary.And:
                return f'{paren(arg1)} and {paren(arg2)}'
            case lex.Binary.Concat:
                return f'{paren(arg1)} ++ {paren(arg2)}'
            case lex.Binary.Take:
                return f'take({pprint(arg1)}, {pprint(arg2)})'
            case lex.Binary.Drop:
                return f'drop({pprint(arg1)}, {pprint(arg2)})'
            case lex.Binary.Apply:
                return f'{paren(arg1)}({pprint(arg2)})'

            case lex.Unary.Neg:
                return f'-{paren(arg1)}'
            case lex.Unary.Not:
                return f'not {paren(arg1)}'
            case lex.Unary.ToInt:
                return f'to_int({pprint(arg1)})'
            case lex.Unary.ToStr:
                return f'to_str({pprint(arg1)})'

            case lex.Ternary.If:
                return f'if ({pprint(arg1)}) {{ {pprint(arg2)} }} else {{ {pprint(arg3)} }}'

            case _:
                if isinstance(tok, lex.Lambda):
                    return f'λ {tok.variable}. {paren(arg1)}'
                elif isinstance(tok, lex.Var):
                    return f'{tok.variable}'
                elif isinstance(tok, (int, str, bool)):
                    return repr(tok)

                assert False, f'Unknown token tok={tok}, type={type(tok)}'
    tokens = np.array(tokens, dtype=np.dtype(object))


def pretty_rename(tokens):
    available_names = [
        'x', 'y', 'z', 'w', 'u', 'v', 's', 't', 'a', 'b', 'c', 'd'
    ]

    mapping = {}

    for token in tokens:
        if isinstance(token, (lex.Lambda, lex.Var)):
            var_name = token.variable
            if var_name not in mapping and available_names:
                mapping[var_name] = available_names.pop()

            token.variable = mapping.get(var_name, f'#{var_name}')


def main():
    parser = ArgumentParser('eval')
    parser.add_argument('icfp', nargs='?')
    parser.add_argument('--stdin', action='store_true')
    parser.add_argument('--file', '-f')
    parser.add_argument('--pprint', action='store_true')

    args = parser.parse_args()

    assert 1 == (args.icfp is not None) + args.stdin + (args.file is not None), (
        "Only one allowed: either `request_string` or `--stdin` or `--file`"
    )

    if args.stdin:
        icfp = sys.stdin.read()
    elif args.file is not None:
        icfp = open(args.file).read()
    else:
        icfp = args.icfp

    if args.pprint:
        tokens = lex.parse_prog(icfp)
        pretty_rename(tokens)
        print(pprint(tokens))
    else:
        tokens = lex.parse_prog(icfp)
        tokens = np.array(tokens, dtype=np.dtype(object))
        result = eval_tokens(tokens, {})
        print(result)


if __name__ == '__main__':
    main()

