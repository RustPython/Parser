"""뱀발, 畵蛇添足, The legs on the snake.

To make it sprint.
Let's start with walking.
"""

import ast
import _ast

import rustpython_parser_pyo3

orig = _ast.AST
        
class ASTType(type):
    def __instancecheck__(self, instance):
        return isinstance(instance, (orig, rustpython_parser_pyo3.unlocated_ast.AST))

class AST(ast.AST, metaclass=ASTType):
    pass
    
ast.AST = AST
