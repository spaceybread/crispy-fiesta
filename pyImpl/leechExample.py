from pyFuzzImpl import LeechFuzzyExtractor
from furbyMatchingImpl import FuzzyMatching

# init the fuzzy extractor
FE = LeechFuzzyExtractor(3)

initial = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4]

# differs by very little compared to inital
close = [3, 1, 5, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 5, 6, 2, 6, 4]

# very different from initial
far = [1, 2, 3] * 8

# get helper and point (as a hash)
b, e = FE.genFromVector(initial)
print(e)

# compute points (as a hash)
c1 = FE.recovFromVector(b, close)
c2 = FE.recovFromVector(b, far)

print(c1)
print(c2)

# compare hashes
print(e == c1, e == c2)

# since FuzzyMatching accepts FuzzyExtractors as an argument,
# it is possible to just pass the Leech FE into it

FM = FuzzyMatching(FE)

S_A = [initial, [999] * 24]
S_B = [close, far]

keys, helpers = FM.makeHelpers(S_A)
rec = FM.attemptMatching(helpers, S_B)
final = FM.returnMatchesAsHashes(rec, S_A, keys)

print(final)