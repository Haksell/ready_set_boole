from itertools import product
from srcs.ex03 import eval_formula
from srcs.ex05 import negation_normal_form
from srcs.utils import get_letters, is_nnf


def equivalent_formulas(f1, f2):
    letters = get_letters(f1 + f2)
    for values in product("01", repeat=len(letters)):
        trans = str.maketrans(letters, "".join(values))
        if eval_formula(f1.translate(trans)) != eval_formula(f2.translate(trans)):
            return False
    return True


def check(formula):
    nnf = negation_normal_form(formula)
    assert is_nnf(nnf), (formula, nnf)
    assert equivalent_formulas(formula, nnf), (formula, nnf)


# test explicit values, double negations and everything combined
def test_negation_normal_form():
    check("A")
    check("A!!")
    check("A!!!!")
    check("AB^")
    check("ABCD^^^")
    check("AB=")
    check("ABCD===")
    check("AB>")
    check("ABCD>>>")
    check("ABCD^>=")
    check("AB=C>D^")
    check("AB&!")
    check("A!B!&!")
    check("AB|!")
    check("A!B|!")
    check("AB|C&!")
    check("ABC&&!")
    check("ABC||!")
    check("A!B!C!&|!")
