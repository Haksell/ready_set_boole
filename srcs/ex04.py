from .formula_checks import is_valid_variable_formula
from .utils import get_letters
from .ex03 import eval_formula
from itertools import product


def get_truth_table(formula):
    assert is_valid_variable_formula(formula)
    letters = get_letters(formula)
    table = [[*letters, "="], ["---"] * (len(letters) + 1)]
    for values in product("01", repeat=len(letters)):
        explicit = formula.translate(str.maketrans(letters, "".join(values)))
        result = eval_formula(explicit)
        table.append([*values, "1" if result else "0"])
    return table


def print_truth_table(formula):
    assert is_valid_variable_formula(formula)
    for row in get_truth_table(formula):
        print("|" + "|".join(x.center(3) for x in row) + "|")
