#import matplotlib.pyplot as p
import zupa1

def mand(a, b):
    zp = 0
    c = complex(a, b)
    for i in range(400):
        zn = zp**2 + c
        if abs(zn) >= 2:
            return False
        zp = zn
    return c

def main(x, y):
    A = []
    B = []
    n = 10000

    for i in range(n):
        g1 = 3
        g2 = 0.1/(1.1**x)
        #p1 = -1 - g1/2
        p2 = y - g2/2
        # A.append((i+p1/g1*n)/(n/g1))
        B.append((i+p2/g2*n)/(n/g2))
        

    for j in B:
        aaaaaaa = zupa1.mandmis(-1.0, j)
        if aaaaaaa != "false":
            print(j)
            return [x+1, j]


vars = [1, -0.28661692527754723]

while True:
    vars = main(vars[0], vars[1])

# p.imshow(S)
# p.show()