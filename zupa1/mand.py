import matplotlib.pyplot as p
import zupa1

A = []
B = []
S = []

n = 1000

g1 = 0.00005
g2 = 0.00005
p1 = -1 - g1/2
p2 = 0.2862167 - g2/2
# A.append((i+p1/g1*n)/(n/g1))
# B.append((i+p2/g2*n)/(n/g2))

(A, B) = zu

def mand(a, b):
    c = complex(a, b)
    zp = 0
    for i in range(400):
        zn = zp**2 + c
        if abs(zn) >= 2:
            return (i+20)%200 + 30
        zp = zn
    return 0
		
for j in B:
	ltemp = []
	for i in A:
		ltemp.append(zupa1.mand(i,-j))
	S.append(ltemp)

p.imshow(S)
p.show()