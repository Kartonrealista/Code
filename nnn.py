from matplotlib import pyplot as p

X = [1, 1]
Y = [0, 1]
S = [2, 2]

xp = 1
xn = 1

def gcd(n1, n2):
    if n1 > n2:
        maxd = n2
    else:
        maxd = n1
    for i in range(int(maxd), 0, -1):
        print(i)
        if n1%i == 0 and n2%i == 0:
            return i

for i in range(2, 1000):
    X.append(i)
    S.append(2)
    if gcd(i, xp) == 1:
        xn = xp + i +1
    else:
        xn = xp/gcd(i, xp)
    Y.append(xn)
    xp = xn

p.scatter(X, Y, S)
p.show()