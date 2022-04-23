import random as ran
import matplotlib.pyplot as plt
import copy


def Board(N):
    brd = []
    d1 = {}
    for i in range(N):
        temp = []
        dtemp = {}
        for j in range(N):
            temp.append(0)
            dtemp[j] = 0
        brd.append(temp)
        d1[i] = dtemp
    return brd, d1

def main(Ratio):
    M = 10
    bwrite, bd = Board(M)
    def randistrib():
        keys1 = list(bd.keys())
        rand1: int = ran.choice(keys1)
        val: dict = bd[rand1]
        keys2 = list(val.keys())
        rand2 = ran.choice(keys2)
        val[rand2] = None
        bwrite[rand1][rand2] = 255
        if bd[rand1].values() == None:
            bd[rand1] = None

        
    for a in range(int(Ratio*M**2)):
        randistrib()

    def sasiad(plansza, cont, a, b):
        if plansza[a][b] == cont and a == (M-1):
            print("yay")
            plt.imshow(bwrite2)
            #plt.show()
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