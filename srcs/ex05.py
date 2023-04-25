from string import ascii_uppercase
from .utils import is_valid_formula


def is_nnf(formula):
    if type(formula) != str or not set(formula) <= set(ascii_uppercase + "!&|"):
        return False
    return True


def negation_normal_form(formula):
    assert is_valid_formula(formula)
    return formula
