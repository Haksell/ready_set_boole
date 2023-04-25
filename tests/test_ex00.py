from ..srcs.ex00 import adder
from ..srcs.utils import UINT_MAX
from random import randint


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
