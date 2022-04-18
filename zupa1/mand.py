import matplotlib.pyplot as p
import zupa1

S = []

n = 1000
g1 = 0.05
g2 = 0.05
p1 = -1 - g1/2
p2 = 0.17509558888888851 - g2/2

(A, B) = zupa1.gen(n, g1, g2, p1, p2)

# def mand(a, b):
#     c = complex(a, b)
#     zp = 0
#     for i in range(400):
#         zn = zp**2 + c
#         if abs(zn) >= 2:
#             return (i+20)%200 + 30
#         zp = zn
#     return 0
		
for j in B:
	ltemp = []
	for i in A:
		ltemp.append(zupa1.mand(i,-j))
	S.append(ltemp)

p.imshow(S)
p.show()