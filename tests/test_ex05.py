from itertools import product
from srcs.ex03 import eval_formula
from srcs.ex05 import is_nnf, negation_normal_form
from srcs.utils import get_letters, is_valid_formula


def check(formula):
    nnf = negation_normal_form(formula)
    assert is_nnf(nnf)
    letters = get_letters(formula)
    for values in product("01", repeat=len(letters)):
        trans = str.maketrans(letters, "".join(values))
        assert eval_formula(nnf.translate(trans)) == eval_formula(
            formula.translate(trans)
        )


def test_subject():
    check("AB&!")
    check("AB|!")
    check("AB>")
    check("AB=")
    check("AB|C&!")
