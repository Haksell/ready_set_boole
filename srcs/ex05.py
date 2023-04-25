from .utils import is_valid_formula


def is_nnf(formula):
    if not is_valid_formula(formula):
        return False
    if any(not c.isupper() or c not in "!&|" for c in formula):
        return False
    return True


def negation_normal_form(formula):
    assert is_valid_formula(formula)
    return formula
