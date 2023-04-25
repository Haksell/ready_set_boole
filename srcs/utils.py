from operator import and_, eq, or_, xor

UINT_MAX = 0xFFFFFFFF
BINARY_OPERATIONS = {
    "|": or_,
    "&": and_,
    "^": xor,
    ">": lambda a, b: not a or b,
    "=": eq,
}


def get_letters(formula):
    return "".join(sorted(set(filter(str.isupper, formula))))
