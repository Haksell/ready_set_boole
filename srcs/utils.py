from operator import and_, eq, or_, xor
from string import ascii_uppercase


UINT_MAX = 0xFFFFFFFF
BINARY_OPERATIONS = {
    "|": or_,
    "&": and_,
    "^": xor,
    ">": lambda a, b: not a or b,
    "=": eq,
}
BINARY_OPERATORS = "".join(BINARY_OPERATIONS.keys())
BOOLEANS = "01"
VALID_CHARACTERS = set(ascii_uppercase + BOOLEANS + BINARY_OPERATORS + "!")


def get_letters(formula):
    return "".join(sorted(set(filter(str.isupper, formula))))


def looks_valid(formula):
    return set(formula) <= VALID_CHARACTERS


def is_valid_formula(formula):
    if not type(formula) == str or not looks_valid(formula):
        return False
    n = 0
    for c in formula:
        n += (c in BOOLEANS or c.isupper()) - (c in BINARY_OPERATORS)
        if n <= 0:
            return False
    return n == 1


def is_valid_explicit_formula(formula):
    return is_valid_formula(formula) and all(not c.isupper() for c in formula)


def is_nnf(formula):
    return formula in tuple(BOOLEANS) or (
        is_valid_formula(formula)
        and all(c.isupper() or c in "!&|" for c in formula)
        and all(a.isupper() or b != "!" for a, b in zip(formula, formula[1::]))
    )
