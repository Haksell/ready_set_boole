from operator import and_, eq, or_, xor
import pytest

OPERATIONS = {"|": or_, "&": and_, "^": xor, ">": lambda a, b: not a or b, "=": eq}


def boolean_evaluation(s):
    assert type(s) == str and set(s) <= set("01!&|^>=")
    stack = []
    for c in s:
        if c in "01":
            stack.append(c == "1")
        elif c == "!":
            stack.append(not stack.pop())
        else:
            a = stack.pop()
            b = stack.pop()
            stack.append(OPERATIONS[c](b, a))
    assert len(stack) == 1
    return stack[0]


def test_boolean_evaluation_unary():
    assert not boolean_evaluation("0")
    assert boolean_evaluation("1")
    assert boolean_evaluation("0!")
    assert not boolean_evaluation("1!")


def test_boolean_evaluation_binary():
    assert not boolean_evaluation("00|")
    assert boolean_evaluation("01|")
    assert boolean_evaluation("10|")
    assert boolean_evaluation("11|")
    assert not boolean_evaluation("00&")
    assert not boolean_evaluation("01&")
    assert not boolean_evaluation("10&")
    assert boolean_evaluation("11&")
    assert not boolean_evaluation("00^")
    assert boolean_evaluation("01^")
    assert boolean_evaluation("10^")
    assert not boolean_evaluation("11^")
    assert boolean_evaluation("00>")
    assert boolean_evaluation("01>")
    assert not boolean_evaluation("10>")
    assert boolean_evaluation("11>")
    assert boolean_evaluation("00=")
    assert not boolean_evaluation("01=")
    assert not boolean_evaluation("10=")
    assert boolean_evaluation("11=")


def test_boolean_evaluation_complex():
    assert boolean_evaluation("1011||=")
    assert boolean_evaluation("1!1|")
    assert boolean_evaluation("111^^")
    assert not boolean_evaluation("1111^^^")
    assert not boolean_evaluation("000==")
    assert boolean_evaluation("111==")


def test_boolean_evaluation_two_material_implications():
    assert not boolean_evaluation("000>>")
    assert boolean_evaluation("001>>")
    assert boolean_evaluation("010>>")
    assert boolean_evaluation("011>>")
    assert boolean_evaluation("100>>")
    assert boolean_evaluation("101>>")
    assert boolean_evaluation("110>>")
    assert boolean_evaluation("111>>")


def test_boolean_evaluation_error():
    with pytest.raises(AssertionError):
        boolean_evaluation(42)
    with pytest.raises(AssertionError):
        boolean_evaluation("")
    with pytest.raises(IndexError):
        boolean_evaluation("10||")
    with pytest.raises(AssertionError):
        boolean_evaluation("111|")
