import matplotlib.pyplot as p

par1 = -1.476
par2 = 0

A = []
B = []

X = []
Y = []
S = []

n = 1000
g1 = 0.1
g2 = 0.1
p1 = -1 - g1/2
p2 = 0.05641669999999997 - g2/2

for i in range(n):  
    A.append((i+p1/g1*n)/(n/g1))
    B.append((i+p2/g2*n)/(n/g2))
	
def mand(a, b):
    zp = complex(a, b)
    c = complex(par1, par2)
    for i in range(16000):
        zn = zp**2 + c
        if abs(zn) >= 2:
            return (i+20)%200 + 10
        zp = zn
    return 0
		
for j in B:
	ltemp = []
	for i in A:
		ltemp.append(mand(i,-j))
	S.append(ltemp)

p.imshow(S)

p.show()