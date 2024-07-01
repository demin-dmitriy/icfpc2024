def mod(a, b):
    s1 = a // abs(a) if a != 0 else 0
    s2 = b // abs(b)
    return s1 * s2 * (abs(a) % abs(b))


A = 4444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444444

n = A
r = 1

base = 99

pos = 0
s = set([pos])


while n != 0:
    d = n % 10

    # U = -base
    # D = base
    # L = -1
    # R = +1

    pos = pos + mod((d - 2), 2) * base + mod((d - 3), 2)
    print('pos', pos// base, mod(pos + base**2, base))
    s.add(pos)

    # if pos == 0: continue # Alternative way to handle base 99

    b = A
    b_pos = 0

    while b != n:

        if b_pos == pos:
            break

        d = b % 10
        b = b // 10

        b_pos = b_pos + mod((d - 2), 2) * base + mod((d - 3), 2)
    else:
        r = r + 1

    n = n // 10


print(r, s, 'expected', len(s))

