#!/usr/bin/env python3

import sys
import requests
from argparse import ArgumentParser
from pathlib import Path

import lex


def encode_string(msg: str):
    return ''.join(
        chr(lex.alphabet.index(i) + 33)
        for i in msg
    )


def post(msg: str):
    return requests.post(
        'https://boundvariable.space/communicate',
        headers={'Authorization': 'Bearer e8de0398-92d9-41d6-9bf4-127b974095c2'},
        data=msg,
    ).text


def com_raw(msg: str):
    return post('S' + encode_string(msg))


def com(msg :str):
    prog = com_raw(msg)
    return lex.parse_prog(prog)


def main():
    parser = ArgumentParser('com', description="Example usage: py/com.py 'get index'")
    parser.add_argument('request_string')
    parser.add_argument('--save', action='store_true')
    parser.add_argument('--raw', action='store_true')
    parser.add_argument('--stdin', action='store_true')
    args = parser.parse_args()

    if args.stdin:
        request_string = sys.stdin.read()
    else:
        request_string = args.request_string

    if args.raw:
        result = com_raw(request_string)
    else:
        tokens = com(request_string)
        result = lex.tokens_to_str(tokens)


    # print(tokens)
    print(result)

    if args.save:
        (Path(__file__).parent / 'history' / request_string).write_text(lex.tokens_to_str(tokens))


if __name__ == '__main__':
    main()
