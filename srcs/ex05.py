import re
from .utils import BOOLEANS, is_valid_formula


def remove_nonstandard_operations(formula):
    stack = []
    for c in formula:
        if c in BOOLEANS or c.isupper():
            stack.append(c)
        elif c == "!":
            stack.append(stack.pop() + "!")
        else:
            b = stack.pop()
            a = stack.pop()
            if c == "^":
                stack.append(a + b + "!&" + a + "!" + b + "&|")
            elif c == "=":
                stack.append(a + b + "&" + a + "!" + b + "!&|")
            elif c == ">":
                stack.append(a + "!" + b + "|")
            else:
                stack.append(a + b + c)
    return stack[0]


def remove_explicit_values(formula):
    return formula


def apply_de_morgan_laws(formula):
    return formula
    # stack = []
    # for c in formula:
    #     if c in BOOLEANS or c.isupper():
    #         stack.append(c)
    #     elif c == "!":
    #         top = stack.pop()
    #         stack.append(top + "!")
    #     else:
    #         b = stack.pop()
    #         a = stack.pop()
    #         stack.append(a + b + c)
    # return stack[0]


def remove_double_negation(formula):
    return re.sub(r"!!", "", formula)


def negation_normal_form(formula):
    assert is_valid_formula(formula)
    formula = remove_double_negation(formula)
    formula = remove_nonstandard_operations(formula)
    formula = remove_double_negation(formula)
    formula = remove_explicit_values(formula)
    formula = remove_double_negation(formula)
    formula = apply_de_morgan_laws(formula)
    formula = remove_double_negation(formula)
    return formula
