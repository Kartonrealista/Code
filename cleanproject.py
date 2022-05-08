import random as ran
import matplotlib.pyplot as plt


def main(ratio):
    class Forest:
        def __init__(self, x):
            self.parent = x
            self.x = x
            self.rank = 0

        def Find(x):
            temp_parent = x.parent
            temp_x = x.x
            # print(temp_parent, temp_x)
            while temp_parent != temp_x:
                temp_x = globals()[f"T{temp_x}"].parent
                temp_parent = globals()[f"T{temp_x}"].parent
                # print(temp_parent, temp_x)
            # Zwraca "najstarszego przodka" instancji x
            return globals()[f"T{temp_x}"]

        def Union(x, y):
            x = Forest.Find(x)
            y = Forest.Find(y)

            # x i y mają tą samą wartość, czyli prawdopodobnie są nawet tą samą instancją. Nie robimy nic
            if x.x == y.x:
                return
            # sortowanie według rangi - czy pierwszy, czy drugi argument zostanie rodzicem
            if x.rank < y.rank:
                x, y = y, x
            # nie wiem, czy to potrzebne, ale nie chcę popsuć kodu. Jeśli instancja została wywołana funkcją Union,
            # to rośnie jej ranga
            x.rank += 1
            y.rank += 1
            # po możliwej zamianie opisanej dwa komentarze wyżej ygrek przyjmuje iksa za rodzica
            y.parent = x.x
            # funkcja jest jednokierunkowa, tu wymuszamy ten kierunek. Ma to większy sens w połączeniu
            # z kodem wywołującym funkcję Union
            if x.rank == y.rank:
                x.rank = x.rank + 1

    def Board(N):
        brd = []
        for i in range(N):
            temp = []
            for j in range(N):
                temp.append(0)
            brd.append(temp)
        return brd

    def pair_to_index(i, j, N):
        return i + j*N

    def index_to_pair(id, N):
        j = id % N
        i = id//N
        return (i, j)

    def random_con(frac_N, l: list):
        sampled_list = ran.sample(l, frac_N)
        return sampled_list

    M = 30
    #bwrite = Board(M)
    blist = []
    for j in range(M):
        for i in range(M):
            temp_id = pair_to_index(i, j, M)
            blist.append(temp_id)

    sampled_blist = random_con(int(ratio*M**2), blist)

    # print(sampled_blist)
    first_row = []
    last_row = []
    for id in sampled_blist:
        # (i, j) = index_to_pair(id, M)
        # bwrite[i][j] = 255
        globals()[f"T{id}"] = Forest(id)
        if id < M:
            # bwrite[i][j] = 80
            first_row.append(id)
            ##
            # bez tego symulacja się partoli. Jest tu po to, żeby przypadkiem nie wyszło, że jakaś
            # instancja klasy odpowiadająca elementowi z rzędu i = 1 (czyli drugiego) nie miała
            # wyższej rangi niż instancja odpow. elementowi z rzędu i = 0 (pierwszego) i nie
            # "przyciągała" do siebie innych instancji jako "dzieci", zanim może to zrobić wartość z góry
            globals()[f"T{id}"].rank += 10*M
        elif id >= M**2-M:
            # bwrite[i][j] = 30
            last_row.append(id)

    # print(first_row)
    # print(last_row)

    def sasiad(id, id2):
        (i, j) = index_to_pair(id, M)
        (k, l) = index_to_pair(id2, M)
        if i-k == 1 and j == l:
            return True
        elif k-i == 1 and j == l:
            return True
        elif i == k and j-l == 1:
            return True
        elif i == k and l-j == 1:
            return True
        else:
            return False

    for id in sorted(sampled_blist):
        for id2 in sorted(sampled_blist):
            if id <= id2 and sasiad(id, id2):
                Forest.Union(globals()[f"T{id}"], globals()[f"T{id2}"])
                # print(globals()[f"T{id}"].parent, globals()[f"T{id}"].x, globals()[
                #    f"T{id2}"].parent, globals()[f"T{id2}"].x)

    for id2 in last_row:
        for id in first_row:
            if id == Forest.Find(globals()[f"T{id2}"]).x:
                # print("Perkolacja!")
                return 1
                # plt.imshow(bwrite)
                # plt.show()
                # exit()
    return 0
    # plt.imshow(bwrite)
    # plt.show()


l = []
l2 = []
for i in range(0, 50):
    l.append(0)
    l2.append(i)
    for j in range(30):
        l[i] += main(0.45+3*i/500)
    print(i)

plt.scatter(l2, l)
plt.show()
