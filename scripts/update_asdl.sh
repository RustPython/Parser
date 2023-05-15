#!/bin/bash
set -e

cd "$(dirname "$(dirname "$0")")"

python ast/asdl_rs.py ast/Python.asdl --ast-dir ast/src/gen/ --module-file ../RustPython/vm/src/stdlib/ast/gen.rs  --pyo3-dir parser-pyo3/src/gen/
rustfmt ast/src/gen/*.rs  ../RustPython/vm/src/stdlib/ast/gen.rs
