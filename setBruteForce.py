from pyFuzzImpl import FuzzyExtractor
from furbyMatchingImpl import FuzzyMatching
import numpy as np

# weird code, unexpected behaviour 

LATTICE = [[1, 0, -1, 2],[2, 3, 1, -1],[1, 1, 2, 0], [0, 1, 1, 3]]

out = []
for _ in range(10):
    FEZ = FuzzyExtractor(LATTICE, 4)
    FM = FuzzyMatching(FEZ)

    S_A = [a for a in range(1000)]
    S_B = [314]

    keys, helpers = FM.makeHelpers(S_A)
    rec = FM.attemptMatching(helpers, S_B)
    final = FM.returnMatches(rec, S_A, keys)

    for a in final:
        if a not in out: 
            out.append((a, _))

print(sorted(out))