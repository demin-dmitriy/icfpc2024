import lex
import eval

import numpy as np

from pathlib import Path
from dataclasses import dataclass
import sys
import math

sys.setrecursionlimit(1500000)
sys.set_int_max_str_digits(100000)


def _reproduce_int_sequence(i: int, alphabet_size: int):
    result = []
    while i != 0:
        d, m = divmod(i, alphabet_size)
        result.append(m)
        i = d

    result.reverse()

    return result


def _compact_int_sequence(sequence: list[int], alphabet_size: int):
    it = iter(sequence)
    result = next(it)
    for s in it:
        result = result * alphabet_size + s

    return result

def _create_alphabet(sequence: list[str]):
    alphabet = list(set(sequence))
    if len(alphabet) == 1:
        assert '*' not in alphabet
        alphabet.append('*')

    if alphabet.index(sequence[0]) == 0:
        alphabet.append(alphabet.pop(0))

    return alphabet


def _compact_sequence(sequence: list[str]) -> tuple[int, list[str]]:
    alphabet = _create_alphabet(sequence)

    result = _compact_int_sequence(
        (
            alphabet.index(s)
            for s in sequence
        ),
        len(alphabet)
    )

    return result, alphabet


def _parse_compact_sequence(compact: int, alphabet: list[str]):
    return [
        alphabet[i]
        for i in _reproduce_int_sequence(compact, len(alphabet))
    ]


def _compact_to_expr(compact: int, alphabet: list[str]):
    alphabet_size = len(alphabet)

    compact_encoded = lex.encode_int(compact)
    alphabet_encoded = lex.encode_string(alphabet)
    alphabet_size_encoded = lex.encode_int(alphabet_size)

    return f'''B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! S B. B$ v" B/ v# I{alphabet_size_encoded} BT I" BD B% v# I{alphabet_size_encoded} S{alphabet_encoded} I{compact_encoded}'''


def compact_string(string: str):
    compact, alphabet = _compact_sequence(string)

    return _compact_to_expr(compact, alphabet)

def convert_string(string: str):
    return 'S' + lex.encode_string(string)


def eval_expr(expr):
    tokens = lex.parse_prog(expr)
    tokens = np.array(tokens, dtype=np.dtype(object))
    result = eval.eval_tokens(tokens, {})

    return result


@dataclass
class Human:
    value: str

    @staticmethod
    def from_file(path: Path) -> 'Human':
        path = Path(path)
        assert path.exists()

        return Human(path.read_text().rstrip())

    def to_icfp(self) -> 'ICFP':
        return ICFP('S' + lex.encode_string(self.value))

    # Генерируется такое число, остатки от деления на кол-во символов которого
    # испльзуются для восстановления исходной последовательности
    def div_mod_compact(self) -> 'ICFP':
        return ICFP(compact_string(self.value))

    def __repr__(self):
        return f'Human({self.value})'


@dataclass
class ICFP:
    value: str

    @staticmethod
    def apply() -> 'ICFP':
        return ICFP('B$')

    @staticmethod
    def and_() -> 'ICFP':
        return ICFP('B&')

    @staticmethod
    def if_() -> 'ICFP':
        return ICFP('?')

    @staticmethod
    def eq_() -> 'ICFP':
        return ICFP('B=')

    @staticmethod
    def str(s: str) -> 'ICFP':
        return ICFP('S' + lex.encode_string(s))

    @staticmethod
    def int(s: str) -> 'ICFP':
        return ICFP('I' + lex.encode_int(s))

    @staticmethod
    def var(index: int) -> 'ICFP':
        return ICFP('v' + lex.encode_int(index))

    @staticmethod
    def lam(index: int) -> 'ICFP':
        return ICFP('L' + lex.encode_int(index))

    @staticmethod
    def from_file(path: Path) -> 'ICFP':
        path = Path(path)
        assert path.exists()

        return ICFP(path.read_text().rstrip())

    def eval(self):
        tokens = lex.parse_prog(self.value)
        tokens = np.array(tokens, dtype=np.dtype(object))

        return eval.eval_tokens(tokens, {})

    def to_human(self) -> Human:
        tokens = lex.parse_prog(self.value)
        tokens = np.array(tokens, dtype=np.dtype(object))

        return Human(eval.eval_tokens(tokens, {}))

    def pprint(self) -> str:
        tokens = lex.parse_prog(self.value)
        eval.pretty_rename(tokens)
        return eval.pprint(tokens)

    def __add__(self, other: 'ICFP') -> 'ICFP':
        return ICFP(f'{self.value} {other.value}')

    def __repr__(self):
        return f'{self.value}'

    def info(self):
        try:
            out = self.eval()
        except Exception as e:
            out = f'error: {e}'

        print(f'[{len(self.value)}]: {self.value}\n[-]: {self.pprint()}\n[{len(out)}]: {out}')



def concat(*strs: ICFP) -> ICFP:
    if len(strs) == 0:
        return ICFP.str('')

    if len(strs) == 0:
        return strs[0]

    result = [
        f'B. {strs[i]} '
        for i in range(len(strs) - 1)
    ] + [
        strs[-1].value
    ]

    return ICFP(
        ''.join(result)
    )


def lambda_concat_times(var_index: int, times: int) -> ICFP:
    lam = ICFP.lam(var_index)
    var = ICFP.var(var_index)

    return lam + concat(*[var] * times)


def apply_lambda(lam: ICFP, arg: ICFP) -> ICFP:
    return ICFP.apply() + lam + arg


def repeat_apply(lam: ICFP, initial_arg: ICFP, times: int) -> ICFP:
    return ICFP(f'{ICFP.apply()} {lam} ' * (times) + initial_arg.value)


def lambda_repeat_apply(var_index: int, apply_lam: ICFP, times: int) -> ICFP:
    lam = ICFP.lam(var_index)
    var = ICFP.var(var_index)
    body = repeat_apply(apply_lam, var, times)

    return lam + body



def let(lhs_index: int, rhs: ICFP, expr: ICFP) -> ICFP:
    lam = ICFP.lam(lhs_index)
    #assert ICFP.var(lhs_index).value in expr.value

    return apply_lambda(
        lam + expr,
        rhs,
    )


def op_equal(lhs: ICFP, rhs: ICFP) -> ICFP:
    return ICFP.eq_() + lhs + rhs

def op_if(condition: ICFP, if_true: ICFP, if_false: ICFP) -> ICFP:
    return ICFP.if_() + condition + if_true + if_false

def op_sub(lhs: ICFP, rhs: ICFP) -> ICFP:
    return ICFP('B-') + lhs + rhs


def y_combinator(self_lambda_index: int, lam: ICFP):
    assert ICFP.var(self_lambda_index).value in lam.value

    return ICFP(f'B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# {ICFP.lam(self_lambda_index)} ' + lam.value)


def repeate_recursive(s: ICFP, times: int, initial: ICFP | None = None):
    f = 65
    vf = ICFP.var(f)

    w = 63
    vw = ICFP.var(w)

    return apply_lambda(
        y_combinator(f,
            ICFP.lam(w) + op_if(op_equal(vw, ICFP.int(0)),
                initial if initial is not None else ICFP.str(''),
                concat(
                    apply_lambda(vf, op_sub(vw, ICFP.int(1))),
                    s,
                )
            )
        ),
        ICFP.int(times),
    )

def lambda_repate_recursive(var_index: int, times: int) -> ICFP:
    lam = ICFP.lam(var_index)
    var = ICFP.var(var_index)

    f = 68
    vf = ICFP.var(f)

    w = 62
    vw = ICFP.var(w)


    return lam + apply_lambda(
        y_combinator(f,
            ICFP.lam(w) + op_if(op_equal(vw, ICFP.int(0)),
                var,
                concat(
                    apply_lambda(vf, op_sub(vw, ICFP.int(1))),
                    var,
                )
            )
        ),
        ICFP.int(times - 1),
    )


def lambda_repeat_at_least(var_index: int, times: int) -> ICFP:
    u = 21
    vu = ICFP.var(u)

    alternatives = [
        lambda_concat_times(var_index, times),
        lambda_repate_recursive(var_index, times),
    ]

    for concat_times in range(1, math.isqrt(times) + 2):
        concat = lambda_concat_times(11, concat_times)

        for repeat_apply_times in range(2, times + 1):
            if concat_times ** repeat_apply_times < times:
                continue

            without_let = lambda_repeat_apply(var_index, concat, repeat_apply_times)
            with_let = let(
                u, concat,
                lambda_repeat_apply(var_index, vu, repeat_apply_times),
            )

            alternatives += [without_let, with_let]

            break

    return min(
        alternatives,
        key=lambda a: len(a.value)
    )


def repeate_string_with_base_exp_at_least(s: str, times: int):
    f = 51
    vf = ICFP.var(f)
    lf = ICFP.lam(f)

    x = 50

    alternatives = []
    for base in range(math.isqrt(times) + 1):
        for exp in range(2, times):
            if base ** exp >= times:
                alternatives.append(
                    apply_lambda(
                        lf + repeat_apply(
                            vf,
                            ICFP.str(s),
                            exp
                        ),
                        lambda_concat_times(x, base)
                    )
                )
                break

    if len(alternatives) == 0:
        print(f'{times} times {len(s)} len str {s}')

    return min(
        alternatives,
        key=lambda a: len(a.value)
    )


def repeate_string_at_least(s: str, times: int) -> ICFP:
    x = 49

    smallest_string = s * times
    alternatives = [
        ICFP.str(smallest_string),
        Human(smallest_string).div_mod_compact(),
    ]

    for i in range(1, times + 1):
        new_s = s * i
        new_t = math.ceil(len(smallest_string) / len(new_s))

        alternatives.append(
            apply_lambda(
                lambda_repeat_at_least(x, new_t),
                ICFP.str(new_s)
            )
        )


        if new_t > 3:
            alternatives.append(
                repeate_string_with_base_exp_at_least(new_s, new_t)
            )


    return min(
        alternatives,
        key=lambda a: len(a.value)
    )


def find_patterns(s: str) -> list[tuple[str, int]]:
    parts = [] # (d, count)
    for d in s:
        if len(parts) == 0 or parts[-1][0] != d:
            parts.append([d, 1])
        else:
            parts[-1][1] += 1

    parts2 = []
    i = 0
    while i < len(parts):
        if i + 3 < len(parts) and parts[i] == parts[i + 2] and parts[i + 1] == parts[i + 3]:
            pattern = parts[i][0] * parts[i][1] + parts[i + 1][0] * parts[i + 1][1]
            repeated = 1
            i += 2
            while i + 1 < len(parts) and pattern == parts[i][0] * parts[i][1] + parts[i + 1][0] * parts[i + 1][1]:
                repeated += 1
                i += 2

            parts2.append([pattern, repeated])

        if i < len(parts):
            parts2.append(parts[i])
        i += 1

    parts3 = []
    i = 0
    while i < len(parts2):
        if i + 1 == len(parts2):
            parts3.append(parts2[i])
            i+=1
        elif parts2[i][1] == 1 and parts2[i + 1][1] == 1:
            parts3.append([parts2[i][0] + parts2[i + 1][0], 1])
            i+=2
        elif parts2[i][1] == 1 and len(parts3) != 0 and parts3[-1][1] == 1:
            parts3[-1][0] += parts2[i][0]
            i+=1
        else:
            parts3.append(parts2[i])
            i+=1

    return parts3


def reproduce_pattern(pattern: list[tuple[str, int]]) -> ICFP:
    y = lex.parse_int('y')
    vy = ICFP.var(y)
    ly = ICFP.lam(y)

    x = lex.parse_int('x')
    vx = ICFP.var(x)
    lx = ICFP.lam(x)

    f = lex.parse_int('f')
    vf = ICFP.var(f)
    lf = ICFP.lam(f)


    repeat_str_x_times = ly + y_combinator(f,
        lx + op_if(op_equal(vx, ICFP.int(0)),
            ICFP.str(''),
            concat(
                apply_lambda(vf, op_sub(vx, ICFP.int(1))),
                vy,
            )
        )
    )

    def select_encode(string, repeate_times):
        with_lambda = apply_lambda(
            apply_lambda(
                vf,
                ICFP.str(string)
            ),
            ICFP.int(repeate_times)
        )

        without_lambda = ICFP.str(string * repeate_times)

        if len(with_lambda.value) < len(without_lambda.value):
            return with_lambda

        return without_lambda


    return let(f, repeat_str_x_times,
        concat(*[
            select_encode(string, repeate_times)
            for string, repeate_times in pattern
        ])
    )


def compact_string_by_patterns(s: str):
    pattern = find_patterns(s)
    return reproduce_pattern(pattern)
