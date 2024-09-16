from pyFuzzImpl import LeechFuzzyExtractor

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
