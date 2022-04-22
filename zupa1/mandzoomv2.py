import matplotlib.pyplot as plt
import zupa1

# def zet(c, depth):
#     z0 = 0
#     for i in range(depth):
#         zn = z0**2 + c

#         if abs(zn) >= 2:
#             return i + 50
#         else:
#             z0 = zn
#     return 0


def main(r1, r2, x1, y2, file_n, depth):

    N = 1000
    A = []
    B = []
    X = []

    x1c = x1 - r1/2
    y2c = y2 - r2/2

    for i in range(N):

        A.append((i + x1c/r1*N)/(N/r1))
        B.append((i + y2c/r2*N)/(N/r2))

    for b in B:
        tempL = []
        for a in A:

            #print(f"mandelbrot{file_n} {100*(b-B[0])/(B[-1]-B[0])}%")
            tempL.append(zupa1.mand(a, -b))
        X.append(tempL)

    plt.imsave(f"/home/kartonrealista/Code/zupa1/zooman2/zooman{file_n}.png", X)


for i in range(50):
    main(3/1.5**(i+1), 3/1.5**(i+1), -1.0, -0.28661692527754723, i, 300)
