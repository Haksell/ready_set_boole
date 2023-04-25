from .utils import is_valid_explicit_formula
from .utils import BINARY_OPERATIONS


def eval_formula(formula):
    assert is_valid_explicit_formula(formula)
    stack = []
    for c in formula:
        if c in "01":
            stack.append(c == "1")
        elif c == "!":
            stack.append(not stack.pop())
        else:
            a = stack.pop()
            b = stack.pop()
            stack.append(BINARY_OPERATIONS[c](b, a))
    return stack[0]
