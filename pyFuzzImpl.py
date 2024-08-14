import random as rdm
import hashlib as hl
import numpy as np

class FuzzyExtractor: 
    def __init__(self, lattice, lattice_dim, k=9):
        # the lattice is defined by a 2D numpy array [dim x dim] where lattice[i] represents a basis vector
        self.lattice = lattice 
        self.dim = lattice_dim
        self.k = k
    
    def gen(self, w):
        es = np.zeros(self.dim, dtype=int)
        ws = np.zeros(self.dim, dtype=int)

        for i in range(self.dim):
            for j in range(self.dim):
                c = rdm.randint(1, self.k)
                es[j] += self.lattice[i][j] * c

        for i in range(self.dim):
            ws[i] = w % 10
            w = w // 10
        ws = ws[::-1]

        bs = es - ws 
        # vector operation

        #print(ws)
        #print(es)
        #print(bs)

        #e = 0
        #for i in range(self.dim):
        #    e = e * 10 + es[i]
        
        #m = hl.sha256()
        #m.update(e.to_bytes(2, 'big'))
        # print(e, e.to_bytes(2, 'big'), m.digest())
        # this is for later!

        return bs, es

    def recov(self, bs, w_p):
        e_prime = np.zeros(self.dim, dtype=int)
        
        ws = np.zeros(self.dim, dtype=int)
        for i in range(self.dim):
            ws[i] = w_p % 10
            w_p = w_p // 10
        ws = ws[::-1]

        e_prime = ws + bs
        # vector operation
         
        return e_prime


lat = np.eye(4)

fe = FuzzyExtractor(lat, 4)
s, e = fe.gen(314)
ep = fe.recov(s, 315)
print(e, ep)
