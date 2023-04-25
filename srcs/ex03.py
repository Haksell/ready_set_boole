from operator import and_, eq, or_, xor

OPERATIONS = {"|": or_, "&": and_, "^": xor, ">": lambda a, b: not a or b, "=": eq}
ALLOWED_CHARACTERS = set("01!|&>=^")


def eval_formula(formula):
    assert type(formula) == str and set(formula) <= ALLOWED_CHARACTERS
    assert all(not c.isupper() for c in formula)
    stack = []
    for c in formula:
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
