from srcs.ex03 import eval_formula
import pytest


def test_eval_formula_unary():
    assert not eval_formula("0")
    assert eval_formula("1")
    assert eval_formula("0!")
    assert not eval_formula("1!")


def test_eval_formula_binary():
    assert not eval_formula("00|")
    assert eval_formula("01|")
    assert eval_formula("10|")
    assert eval_formula("11|")
    assert not eval_formula("00&")
    assert not eval_formula("01&")
    assert not eval_formula("10&")
    assert eval_formula("11&")
    assert not eval_formula("00^")
    assert eval_formula("01^")
    assert eval_formula("10^")
    assert not eval_formula("11^")
    assert eval_formula("00>")
    assert eval_formula("01>")
    assert not eval_formula("10>")
    assert eval_formula("11>")
    assert eval_formula("00=")
    assert not eval_formula("01=")
    assert not eval_formula("10=")
    assert eval_formula("11=")


def test_eval_formula_complex():
    assert eval_formula("1011||=")
    assert eval_formula("1!1|")
    assert not eval_formula("1111^^^")


def test_eval_formula_two_ands():
    assert not eval_formula("000&&")
    assert not eval_formula("001&&")
    assert not eval_formula("010&&")
    assert not eval_formula("011&&")
    assert not eval_formula("100&&")
    assert not eval_formula("101&&")
    assert not eval_formula("110&&")
    assert eval_formula("111&&")


def test_eval_formula_two_ors():
    assert not eval_formula("000||")
    assert eval_formula("001||")
    assert eval_formula("010||")
    assert eval_formula("011||")
    assert eval_formula("100||")
    assert eval_formula("101||")
    assert eval_formula("110||")
    assert eval_formula("111||")


def test_eval_formula_two_xors():
    assert not eval_formula("000^^")
    assert eval_formula("001^^")
    assert eval_formula("010^^")
    assert not eval_formula("011^^")
    assert eval_formula("100^^")
    assert not eval_formula("101^^")
    assert not eval_formula("110^^")
    assert eval_formula("111^^")


def test_eval_formula_two_implications():
    assert eval_formula("000>>")
    assert eval_formula("001>>")
    assert eval_formula("010>>")
    assert eval_formula("011>>")
    assert eval_formula("100>>")
    assert eval_formula("101>>")
    assert not eval_formula("110>>")
    assert eval_formula("111>>")


def test_eval_formula_two_equivalences():
    assert not eval_formula("000==")
    assert eval_formula("001==")
    assert eval_formula("010==")
    assert not eval_formula("011==")
    assert eval_formula("100==")
    assert not eval_formula("101==")
    assert not eval_formula("110==")
    assert eval_formula("111==")


def test_eval_formula_error():
    with pytest.raises(AssertionError):
        eval_formula(42)
    with pytest.raises(AssertionError):
        eval_formula("")
    with pytest.raises(AssertionError):
        eval_formula("10||")
    with pytest.raises(AssertionError):
        eval_formula("111|")
