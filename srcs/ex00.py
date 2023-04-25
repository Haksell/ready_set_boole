def adder(a, b):
    res = carry = 0
    for i in range(32):
        da = a >> i & 1
        db = b >> i & 1
        res |= (da ^ db ^ carry) << i
        carry = (da & db) | (da & carry) | (db & carry)
    return res
