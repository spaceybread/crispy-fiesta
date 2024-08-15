from pyFuzzImpl import FuzzyExtractor
import numpy as np

lat = [
    [1, 0, -1, 2],
    [2, 3, 1, -1],
    [1, 1, 2, 0], 
    [0, 1, 1, 3]
]

fe = FuzzyExtractor(lat, 4)
s, e = fe.gen(314)

for i in range(9):
    for j in range(9):
        for k in range(9):
            n = i * 100 + j * 10 + k
            ep = fe.recov(s, n)
            if np.array_equal(e, ep):
                print(n, e, ep, True)
            
