from srcs.ex03 import eval_formula
from itertools import product
from string import ascii_uppercase

ALLOWED_CHARACTERS = set(ascii_uppercase + "!&|^>=")


def get_truth_table(s):
    letters = "".join(sorted(set(filter(str.isupper, s))))
    table = [[*letters, "="], ["---"] * (len(letters) + 1)]
    for values in product("01", repeat=len(letters)):
        formula = s.translate(str.maketrans(letters, "".join(values)))
        result = eval_formula(formula)
        table.append([*values, "1" if result else "0"])
    return table


def print_truth_table(s):
    for row in get_truth_table(s):
        print("|" + "|".join(x.center(3) for x in row) + "|")
