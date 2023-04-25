UINT_MAX = 0xFFFFFFFF


def get_letters(formula):
    return "".join(sorted(set(filter(str.isupper, formula))))


def is_valid_formula(formula):
    return type(formula) == str and all(c.isupper() or c in "!&|^>=" for c in formula)
