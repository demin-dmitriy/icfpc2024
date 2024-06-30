
A = 3123


B = A
C = 0

while B != 0:
    d = B % 10
    B = B // 10
    C = C * 10 + d


print(A == C)


