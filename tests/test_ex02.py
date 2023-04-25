from ..srcs.ex02 import gray_code


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
    assert gray_code(9) == 13
    assert gray_code(10) == 15
    assert gray_code(11) == 14
    assert gray_code(12) == 10
    assert gray_code(13) == 11
    assert gray_code(14) == 9
    assert gray_code(15) == 8
    assert gray_code(16) == 24
    assert gray_code(17) == 25
    assert gray_code(18) == 27
    assert gray_code(19) == 26
    assert gray_code(20) == 30
