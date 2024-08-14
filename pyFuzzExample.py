from pyFuzzImpl import FuzzyExtractor
import numpy as np

lat = np.eye(4)
# standard basis, there's no point to using
# this 

lat = [
    [1, 0, -1, 2],
    [2, 3, 1, -1],
    [1, 1, 2, 0], 
    [0, 1, 1, 3]
]
# I just picked some small values randomly

fe = FuzzyExtractor(lat, 4)
s, e = fe.gen(314)

# close
ep = fe.recov(s, 315)
print(e, ep, np.array_equal(e, ep))

# close
ep = fe.recov(s, 414)
print(e, ep, np.array_equal(e, ep))

# close
ep = fe.recov(s, 203)
print(e, ep, np.array_equal(e, ep))

# far
ep = fe.recov(s, 168)
print(e, ep, np.array_equal(e, ep))

# far
ep = fe.recov(s, 311)
print(e, ep, np.array_equal(e, ep))

# far
ep = fe.recov(s, 300)
print(e, ep, np.array_equal(e, ep))



# I should probably look into what
# other values are 'close enough' 
# and how I would make this work
# with non-numeric values

# probably just encoding other info
# as numbers ¯\_(ツ)_/¯