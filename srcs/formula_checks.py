from string import ascii_uppercase
from .utils import BINARY_OPERATIONS

BINARY_OPERATORS = "".join(BINARY_OPERATIONS.keys())
BOOLEANS = "01"


def is_valid_formula(formula):
    if not type(formula) == str or not set(formula) <= set(
        ascii_uppercase + BINARY_OPERATORS + BOOLEANS + "!"
    ):
        return False
    n = 0
    for c in formula:
        n += (c in BOOLEANS or c.isupper()) - (c in BINARY_OPERATORS)
        if n <= 0:
            return False
    return n == 1


def is_valid_explicit_formula(formula):
    return is_valid_formula(formula) and all(not c.isupper() for c in formula)


def is_valid_variable_formula(formula):
    return is_valid_formula(formula) and all(c not in BOOLEANS for c in formula)


def is_valid_nnf(formula):
    return formula in tuple(BOOLEANS) or (
        is_valid_variable_formula(formula)
        and all(a.isupper() or b != "!" for a, b in zip(formula, formula[1::]))
    )
