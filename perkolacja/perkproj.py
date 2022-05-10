import perkolacja as prk
import matplotlib.pyplot as plt

# l = [0]
# l2 = [0]
# for i in range(1, 901):
#     l.append(0)
#     l2.append(i/9)
#     for j in range(30):
#         l[i] += prk.perk_hex(i/900)
#     print(i)
# l2, l = prk.perkhist()
prk.perk_sq_new()
plt.scatter(l2, l)
plt.show()