from pyFuzzImpl import GaussFuzzyExtractor
import numpy as np

# Gaussian Lattice with scale 2 and 24 dimensions

#for l in lat:
#    print(l)

all = set()

GEN_NUM = 314
#GEN_NUM = int(input())
fe = GaussFuzzyExtractor(16, 2)

for _ in range(10):
    # init at different instances results in some
    # differences, which is expected since 
    # FuzzyExtractor picks random coeffs each time
    # it is init, as such, some values are considered
    # similar sometimes while not close other times

    s, e = fe.gen(GEN_NUM)

    for i in range(10):
        for j in range(10):
            for k in range(10):
                n = i * 100 + j * 10 + k
                ep = fe.recov(s, n)
                if np.array_equal(e, ep):
                    if n not in all:
                        #print(_, n, ep, e)
                        all.add(n)
            
print(sorted(list(all)))
print(len(all))
