#!/usr/bin/env python3

from argparse import ArgumentParser
from enum import Enum
from typing import NamedTuple


alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\]^_`|~ \n"


class Op(Enum):
    pass

class Ternary(Op):
    If = 1

    def __str__(self):
        return self.__repr__()

    def __repr__(self):
        return self.name


class Binary(Op):
    Add = '+'   # Integer addition    B+ I# I$ -> 5
    Sub = '-'   # Integer subtraction    B- I$ I# -> 1
    Mul = '*'   # Integer multiplication    B* I$ I# -> 6
    Div = '/'   # Integer division (truncated towards zero)    B/ U- I( I# -> -3
    Mod = '%'   # Integer modulo    B% U- I( I# -> -1
    Less = '<'   # Integer comparison    B< I$ I# -> false
    Greater = '>'   # Integer comparison    B> I$ I# -> true
    Eq = '='   # Equality comparison, works for int, bool and string    B= I$ I# -> false
    Or = '|'   # Boolean or    B| T F -> true
    And = '&'   # Boolean and    B& T F -> false
    Concat = '.'   # String concatenation    B. S4% S34 -> "test"
    Take = 'T'   # Take first x chars of string y    BT I$ S4%34 -> "tes"
    Drop = 'D'   # Drop first x chars of string y    BD I$ S4%34 -> "t"
    Apply = '$'   # Apply term x to y (see Lambda abstractions)

    def __str__(self):
        return self.__repr__()

    def __repr__(self):
        return self.value


class Unary(Op):
    Neg = '-' #    Integer negation    U- I$ -> -3
    Not = '!' #    Boolean not    U! T -> false
    ToInt = '#' #    string-to-int: interpret a string as a base-94 number    U# S4%34 -> 15818151
    ToStr = '$' #    int-to-string: inverse of the above    U$ I4%34 -> test

    def __str__(self):
        return self.__repr__()

    def __repr__(self):
        return self.value

class Lambda(NamedTuple):
    variable: int

    def __str__(self):
        return self.__repr__()

    def __repr__(self):
        return f'λ#{self.variable}'


class Var(NamedTuple):
    variable: int

    def __str__(self):
        return self.__repr__()

    def __repr__(self):
        return f'#{self.variable}.'


# unknown token
class Unknown(NamedTuple):
    raw: str

    def __str__(self):
        return self.__repr__()

    def __repr__(self) -> str:
        return f'Unknown({self.raw})'


def parse_source_char(ch: str) -> int:
    return ord(ch) - 33


def parse_int(s: str):
    return sum(94**i * parse_source_char(ch) for i, ch in enumerate(s[::-1]))


def parse_string(s: str):
    return ''.join(
        alphabet[parse_source_char(i)]
        for i in s
    )


def parse_token(tok):
    match tok[0]:
        case 'S':
            return parse_string(tok[1:])
        case '?':
            return Ternary.If
        case 'T':
            return True
        case 'F':
            return False
        case 'I':
            return parse_int(tok[1:])
        case 'B':
            return Binary(tok[1:])
        case 'U':
            return Unary(tok[1:])
        case 'L':
            return Lambda(variable=parse_int(tok[1:]))
        case 'v':
            return Var(variable=parse_int(tok[1:]))

    return Unknown(tok)



def parse_prog(icfp):
    tokens = [parse_token(tok) for tok in icfp.split(' ')]
    return tokens

def tokens_to_str(tokens):
    return ' '.join(str(tok) for tok in tokens)


def main():
    parser = ArgumentParser('lex')
    parser.add_argument('icfp')
    args = parser.parse_args()
    icfp = args.icfp
    print(parse_prog(icfp))
    # print()
    # print(tokens_to_str(parse_prog(icfp)))


if __name__ == '__main__':
    main()

