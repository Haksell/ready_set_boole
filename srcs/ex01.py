from srcs.ex00 import adder


def multiplier(a, b):
    res = 0
    for ia in range(32):
        da = a >> ia & 1
        mask = adder(~da, 1)
        res = adder(res, (b & mask) << ia)
    return res
