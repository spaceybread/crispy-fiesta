import random as rdm
import hashlib as hl
import numpy as np

class FuzzyExtractor: 
    def __init__(self, lattice, lattice_dim, k=9):
        # the lattice is defined by a 2D numpy
        # array [dim x dim] where lattice[i] 
        # represents a basis vector
        self.lattice = lattice 
        self.dim = lattice_dim
        self.k = k

    # take a point in R^n and find the closest
    # lattice point
    def closest(self, p, B):
        z = np.round(np.linalg.solve(B, p))
        return B @ z
    

    # get the vector and the snapped point
    def gen(self, w):
        es = np.zeros(self.dim, dtype=int)
        ws = np.zeros(self.dim, dtype=int)

        # there's probably a clean way to
        # do this with numpy
        for i in range(self.dim):
            for j in range(self.dim):
                c = rdm.randint(1, self.k)
                es[j] += self.lattice[i][j] * c
        
        es = self.closest(es, self.lattice)
        # after finding a random point in R^n, 
        # snap it to some point in the lattice

        for i in range(self.dim):
            ws[i] = w % 10
            w = w // 10
        ws = ws[::-1]

        bs = es - ws 
        return bs, es

    # given some point and the vector, find
    # the closest lattice point
    def recov(self, bs, w_p):
        e_prime = np.zeros(self.dim, dtype=int)
        
        ws = np.zeros(self.dim, dtype=int)
        for i in range(self.dim):
            ws[i] = w_p % 10
            w_p = w_p // 10
        ws = ws[::-1]

        e_prime = ws + bs
        e_final = self.closest(e_prime, self.lattice)
        # after finding the corresponding
        # point, snap it to a lattice
        # point
         
        return e_final


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

# far
ep = fe.recov(s, 168)
print(e, ep, np.array_equal(e, ep))

# I should probably look into what
# other values are 'close enough' 
# and how I would make this work
# with non-numeric values

# probably just encoding other info
# as numbers ¯\_(ツ)_/¯