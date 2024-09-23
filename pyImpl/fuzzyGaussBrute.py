from pyFuzzImpl import GaussFuzzyExtractor
import numpy as np
import random as rdm

# Gaussian Lattice with scale 2 and 24 dimensions

#for l in lat:
#    print(l)

all = set()
GEN_NUM = 3141
fe = GaussFuzzyExtractor(10, 2, 0.5)
s, e = fe.gen(GEN_NUM)
TESTS = 1000

for _ in range(TESTS):
    # init at different instances results in some
    # differences, which is expected since 
    # FuzzyExtractor picks random coeffs each time
    # it is init, as such, some values are considered
    # similar sometimes while not close other times

    n = rdm.randint(3 * GEN_NUM // 4, GEN_NUM + GEN_NUM // 4)
    ep = fe.recov(s, n)
    if e == ep:
        if n not in all:
            #print(_, n, ep, e)
            all.add(n)

print(sorted(list(all)))
print(len(all))
print(len(all) / TESTS)
