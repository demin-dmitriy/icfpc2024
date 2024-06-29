cache = [ None ] * 1234568

def f(a):
    if cache[a] is not None:
        return cache[a]

    if a <= 2:
        return a

    v = a
    divisors = []
    for s in range(2, a):
        if s**2 > a: break

        f_s = f(s)
        if f_s > s - 1 and a % s == 0:
            v = v // f_s * (f_s - 1)

            y = a // s
            if y == s: continue
            f_y = f(y)
            if f_y > y - 1:
                divisors.append(f_y)

    for d in divisors[::-1]:
        v = v // d * (d - 1)

    cache[a] = min(a, 1 + v)
    return cache[a]
