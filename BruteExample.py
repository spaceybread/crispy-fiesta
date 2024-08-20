from pyFuzzImpl import FuzzyExtractor
import numpy as np

lat = [
    [1, 0, -1, 2],
    [2, 3, 1, -1],
    [1, 1, 2, 0], 
    [0, 1, 1, 3]
]

all = set()

GEN_NUM = int(input())

for _ in range(100):
    # init at different instances results in some
    # differences, which is expected since 
    # FuzzyExtractor picks random coeffs each time
    # it is init, as such, some values are considered
    # similar sometimes while not close other times

    fe = FuzzyExtractor(lat, 4)
    s, e = fe.gen(GEN_NUM)

    for i in range(9):
        for j in range(9):
            for k in range(9):
                n = i * 100 + j * 10 + k
                ep = fe.recov(s, n)
                if np.array_equal(e, ep):
                    if n not in all:
                        print(_, n, ep, e)
                        all.add(n)
            
print(sorted(list(all)))