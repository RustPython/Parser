import baembal

import sys
import ast
import _ast

ast.AST = baembal.AST
_ast.AST = baembal.AST

import ast as py_ast
import rustpython_parser_pyo3 as rust_ast


dump = ast.dump

import timeit

from glob import glob


files = {}
for path in glob("../../cpython/Lib/**/*.py"):
    try:
        txt = open(path, 'r').read()
    except UnicodeDecodeError:
        continue
    # try:
    #     if py_ast.dump(py_ast.parse(txt)) != py_ast.dump(rust_ast.parse(txt)):
    #         continue
    # except SyntaxError:
    #     continue
    files[path] = txt


t1 = 0.0
t2 = 0.0
t3 = 0.0

for path, txt in files.items():
    try:
        txt = open(path, 'r').read()
    except UnicodeDecodeError:
        continue

    # p = py_ast.parse(txt)
    # r = rust_ast.parse(txt)

    # compile(p, 'x', 'exec')
    # compile(r, 'x', 'exec')

    # break
    try:
        p = timeit.timeit(lambda: dump(py_ast.parse(txt)), number=10)
        r1 = timeit.timeit(lambda: dump(rust_ast.parse(txt)), number=10)
        r2 = timeit.timeit(lambda: dump(rust_ast.parse_wrap(txt)), number=10)
    except Exception as e:
        print('error:', path, e)
        continue
    t1 += p
    t2 += r1
    t3 += r2
    print(f'accum: {t1:.2f} {t2:.2f} {t3:.2f} {path}')
