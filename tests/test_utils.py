from srcs.utils import (
    is_valid_explicit_formula,
    is_valid_formula,
    is_nnf,
)


def test_is_valid_formula():
    assert is_valid_formula("0")
    assert is_valid_formula("A")
    assert is_valid_formula("A!B!|")
    assert is_valid_formula("0!1!&")
    assert is_valid_formula("A!B|")
    assert is_valid_formula("AB&1!B!&|")
    assert is_valid_formula("A!B!&0!|")
    assert is_valid_formula("AB&!")
    assert is_valid_formula("AB|!")
    assert is_valid_formula("AB>")
    assert is_valid_formula("AB=")
    assert is_valid_formula("AB|C&!")


def test_is_invalid_formula():
    assert not is_valid_formula(1)
    assert not is_valid_formula("")
    assert not is_valid_formula("!")
    assert not is_valid_formula("&")
    assert not is_valid_formula("A&B")
    assert not is_valid_formula("AB!!")
    assert not is_valid_formula("AB^^")
    assert not is_valid_formula("ABC|")
    assert not is_valid_formula("ABC|D&")


def test_is_valid_explicit_formula():
    assert is_valid_explicit_formula("0!1!&")
    assert not is_valid_explicit_formula("A!B!&0!|")
    assert not is_valid_explicit_formula("AB&!")


def test_is_nnf():
    assert is_nnf("A!B!|")
    assert is_nnf("A!B!&")
    assert is_nnf("A!B|")
    assert is_nnf("AB&A!B!&|")
    assert is_nnf("A!B!&C!|")


def test_is_not_nnf():
    assert not is_nnf("AB&!")
    assert not is_nnf("AB|!")
    assert not is_nnf("AB>")
    assert not is_nnf("AB=")
    assert not is_nnf("AB|C&!")
