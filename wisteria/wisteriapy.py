from matplotlib import pyplot as p
import wisteria as w

# X = []
# Y = []
# S = []
# for i in range (1000000):
#     i_list = str(i)
#     prod = 1
#     for j in i_list:
#         if int(j) != 0:
#             prod *= int(j)
#     xval = i - prod
#     Y.append(i)
#     X.append(xval)
#     S.append(2)

(X, Y, S) = w.f1(100000)

p.scatter(Y, X, S)
p.show()