#!/usr/bin/env python3

import requests
from argparse import ArgumentParser

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


def com(msg :str):
    tokens = lex.parse_prog(post('S' + encode_string(msg)))
    return tokens


def main():
    parser = ArgumentParser('com', description="Example usage: py/com.py 'get index'")
    parser.add_argument('request_string')
    args = parser.parse_args()
    print(lex.tokens_to_str(com(args.request_string)))


if __name__ == '__main__':
    main()
