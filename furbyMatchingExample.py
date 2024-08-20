from pyFuzzImpl import FuzzyExtractor
from furbyMatchingImpl import *
import numpy as np

LATTICE = [[1, 0, -1, 2],[2, 3, 1, -1],[1, 1, 2, 0], [0, 1, 1, 3]]

# Initial setup 
FE = FuzzyExtractor(LATTICE, 4)

S_A = [314, 168, 159, 333, 819, 606]
S_B = [413, 68, 69, 358, 359, 444, 555, 818, 700, 717, 999, 819]

keys, helpers = makeHelpers(FE, S_A)
rec = attemptMatching(FE, helpers, S_B)
final = returnMatches(rec, keys)

print(final)
