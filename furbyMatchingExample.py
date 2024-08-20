from pyFuzzImpl import FuzzyExtractor
from furbyMatchingImpl import FuzzyMatching
import numpy as np

LATTICE = [[1, 0, -1, 2],[2, 3, 1, -1],[1, 1, 2, 0], [0, 1, 1, 3]]

# Initial setup 
FEZ = FuzzyExtractor(LATTICE, 4)
FM = FuzzyMatching(FEZ)

S_A = [314, 168, 159, 333, 819, 606]
S_B = [68, 69, 358, 359, 444, 555, 818, 700, 717, 999, 819]

keys, helpers = FM.makeHelpers(S_A)
rec = FM.attemptMatching(helpers, S_B)
final = FM.returnMatches(rec, S_A, keys)

print(final)
