from itertools import product
from srcs.ex03 import eval_formula
from srcs.ex05 import is_nnf, negation_normal_form
from srcs.utils import get_letters


def test_is_nnf_true():
    assert is_nnf("A")
    assert is_nnf("A!B!|")
    assert is_nnf("A!B!&")
    assert is_nnf("A!B|")
    assert is_nnf("AB&A!B!&|")
    assert is_nnf("A!B!&C!|")


def test_is_nnf_false():
    assert is_nnf(42)


# def check(formula):
#     nnf = negation_normal_form(formula)
#     assert is_nnf(nnf)
#     letters = get_letters(formula)
#     for values in product("01", repeat=len(letters)):
#         trans = str.maketrans(letters, "".join(values))
#         assert eval_formula(nnf.translate(trans)) == eval_formula(
#             formula.translate(trans)
#         )


# def test_negation_normal_form_subject():
#     check("AB&!")
#     check("AB|!")
#     check("AB>")
#     check("AB=")
#     check("AB|C&!")
