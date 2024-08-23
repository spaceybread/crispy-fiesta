from pyFuzzImpl import FuzzyExtractor
from furbyMatchingImpl import FuzzyMatching
import numpy as np

LATTICE = [[1, 0, -1, 2],[2, 3, 1, -1],[1, 1, 2, 0], [0, 1, 1, 3]]

out = []

FEZ = FuzzyExtractor(LATTICE, 4)
FM = FuzzyMatching(FEZ)

S_A = [314]
S_B = [i for i in range(1000)]
    

keys, helpers = FM.makeHelpers(S_A)
rec = FM.attemptMatching(helpers, S_B)

# this is a debug version of FM.returnMatches(rec, S_A, keys)
# that returns pairs that were matched
# this is never really be possible because A should
# never have access to S_B
finalDebug = FM.returnMatchesAsPairs(rec, S_A, keys, S_B)
final = FM.returnMatches(rec, S_A, keys)

for l in finalDebug:
    print(*l)

print(final)