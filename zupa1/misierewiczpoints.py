#import matplotlib.pyplot as p
import zupa1

A = []
B = []
S = []
n = 1000000

for i in range(n):
    g1 = 3
    g2 = 0.0001
    p1 = -1 - g1/2
    p2 = 0.2862167 - g2/2
    # A.append((i+p1/g1*n)/(n/g1))
    B.append((i+p2/g2*n)/(n/g2))
	
def mand(a, b):
    zp = 0
    c = complex(a, b)
    for i in range(40000):
        zn = zp**2 + c
        if abs(zn) >= 2:
            return False
        zp = zn
    return c


ltemp = 0

for j in B:
    aaaaaaa = zupa1.mand(-1, j)
    if aaaaaaa != False:
        ltemp = aaaaaaa
        print(ltemp)
        exit()
print(ltemp)

# p.imshow(S)
# p.show()