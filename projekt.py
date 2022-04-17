import random as ran
import matplotlib.pyplot as plt
import copy


def Board(N):
    brd = []
    for i in range(N):
        temp = []
        for j in range(N):
            temp.append(0)
        brd.append(temp)
    return brd

def main(Ratio):
    M = 10
    bwrite = Board(M)
    def randistrib():
        rand1 = ran.randrange(M)
        rand2 = ran.randrange(M)
        if bwrite[rand1][rand2] == 255:
            randistrib
        else:
            bwrite[rand1][rand2] = 255
    for a in range(int(Ratio*M**2)):
        randistrib()

    def sasiad(plansza, cont, a, b):
        if plansza[a][b] == cont and a == (M-1):
            print("yay")
            
            return 1
        if a > 0 and plansza[a - 1][b] != 0 and plansza[a - 1][b] != cont:
            plansza[a - 1][b] = cont
            
        if (M-1) > a and plansza[a + 1][b] != 0 and plansza[a + 1][b] != cont:
            plansza[a + 1][b] = cont
            
        if b > 0 and plansza[a][b-1] != 0 and plansza[a][b-1] != cont:
            plansza[a][b-1] = cont
            
        if (M-1) > b and plansza[a][b+1] != 0 and plansza[a][b+1] != cont:
            plansza[a][b+1] = cont
        
    def sasiady2():
        lizda = []
        for l in range(M): 
            for m in range(M):
                if bwrite2[l][m] == licznik:
                    lizda.append([l,m])
        for lis in lizda:
            retvar = sasiad(bwrite2, licznik, lis[0], lis[1])
            if retvar == 1:
                return 1

    bwrite2 = copy.deepcopy(bwrite)

    licznik = 60
    for k in range(M):
        if bwrite2[0][k] != 0:
            bwrite2[0][k] = licznik
            sasiad(bwrite2, licznik, 0, k)
            for x in range(2*M):
                wyn = sasiady2()
                if wyn == 1:
                    return 1
    return 0

x = 0
H = []
X = []

while x < 101:
    H.append(0)
    X.append(x)
    print(int(x/20))
    for i in range(30):
        wynik = main(x/100)
        H[int(x/20)] += wynik
    x += 20

print(H, X)