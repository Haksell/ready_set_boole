from string import ascii_uppercase


UINT_MAX = 0xFFFFFFFF
BINARY_OPERATORS = "&|^>="


def get_letters(formula):
    return "".join(sorted(set(filter(str.isupper, formula))))


def is_valid_formula(formula):
    if not type(formula) == str or not set(formula) <= set(
        ascii_uppercase + "01!" + BINARY_OPERATORS
    ):
        return False
    n = 0
    for c in formula:
        n += c.isdigit() or c.isupper() - c in BINARY_OPERATORS
        if n <= 0:
            return False
    assert n == 1
