from random import randint
from srcs.ex01 import multiplier
from srcs.utils import UINT_MAX


def test_multiplier_small():
    for a in range(6):
        for b in range(6):
            assert multiplier(a, b) == a * b


def test_multiplier_overflow():
    assert multiplier(UINT_MAX, 2) == UINT_MAX - 1
    assert multiplier(10**5, 10**5) == 10**10 & UINT_MAX


def test_multiplier_random():
    for _ in range(25):
        a = randint(0, UINT_MAX)
        b = randint(0, UINT_MAX)
        assert multiplier(a, b) == a * b & UINT_MAX, f"{a=} {b=}"
