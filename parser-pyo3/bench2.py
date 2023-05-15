import sys
import baembal

import ast
ast.AST = baembal.AST

from ast import dump


arg = sys.argv[1]  # python or rustpython
if arg == "python":
    import ast
elif arg == "rustpython":
    import rustpython_parser_pyo3 as ast
else:
    assert False

from glob import glob

for path in glob("../../cpython/Lib/*.py"):
    try:
        txt = open(path, 'r').read()
    except UnicodeDecodeError:
        continue
    m = ast.parse(txt)
    code = compile(m, path, 'exec')
    # d = dump(m, indent=True)
