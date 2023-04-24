from random import randint
from utils import UINT_MAX


def adder(a, b):
    res = carry = 0
    for i in range(32):
        da = a >> i & 1
        db = b >> i & 1
        res |= (da ^ db ^ carry) << i
        carry = (da & db) | (da & carry) | (db & carry)
    return res


def test_adder_small():
    for a in range(6):
        for b in range(6):
            assert adder(a, b) == a + b


def test_adder_overflow():
    assert adder(UINT_MAX, 3) == 2
    assert adder(UINT_MAX, UINT_MAX) == UINT_MAX - 1
    assert adder(2 * 10**9, 3 * 10**9) == (5 * 10**9) & UINT_MAX


def test_adder_random():
    for _ in range(25):
        a = randint(0, UINT_MAX)
        b = randint(0, UINT_MAX)
        assert adder(a, b) == a + b & UINT_MAX
