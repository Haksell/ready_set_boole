UINT_MAX = 0xFFFFFFFF


def get_letters(formula):
    return "".join(sorted(set(filter(str.isupper, formula))))
