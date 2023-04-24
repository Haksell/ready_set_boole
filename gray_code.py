from functools import cache


@cache
def gray_code(n):
    if n <= 1:
        return n
    largest_bit = 1 << (n.bit_length() - 1)
    return largest_bit | gray_code(~n & (largest_bit - 1))


def test_gray_code():
    assert gray_code(0) == 0
    assert gray_code(1) == 1
    assert gray_code(2) == 3
    assert gray_code(3) == 2
    assert gray_code(4) == 6
    assert gray_code(5) == 7
    assert gray_code(6) == 5
    assert gray_code(7) == 4
    assert gray_code(8) == 12
