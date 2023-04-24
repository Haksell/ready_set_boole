from random import randint


def adder(a, b):
    res = carry = 0
    for i in range(32):
        ai = a >> i & 1
        bi = b >> i & 1
        res |= (ai ^ bi ^ carry) << i
        carry = (ai & bi) | (ai & carry) | (bi & carry)
    return res


UINT_MAX = (1 << 32) - 1


def test_adder_small():
    assert adder(0, 0) == 0
    assert adder(0, 42) == 42
    assert adder(42, 0) == 42
    assert adder(42, 42) == 84
    assert adder(100, 300) == 400


def test_adder_overflow():
    assert adder(UINT_MAX, 3) == 2
    assert adder(UINT_MAX, UINT_MAX) == UINT_MAX - 1
    assert adder(2 * 10**9, 3 * 10**9) == (5 * 10**9) & UINT_MAX


def test_adder_random():
    for _ in range(100):
        a = randint(0, UINT_MAX)
        b = randint(0, UINT_MAX)
        assert adder(a, b) == a + b & UINT_MAX
