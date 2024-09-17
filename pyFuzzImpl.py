import random as rdm
from hashlib import sha256
import numpy as np
from leechImpl import Leech

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
    
class GaussFuzzyExtractor(FuzzyExtractor): 
    def __init__(self, lattice_dim, scale, k = 9):
        # the lattice is defined by a 2D numpy
        # array [dim x dim] where lattice[i] 
        # represents a basis vector
        self.dim = lattice_dim
        self.scale = scale
        self.k = k
        
        lat = []
        for i in range(lattice_dim):
            line = [0] * i + [scale] + [0] * (lattice_dim - 1 - i)
            lat.append(line)

        self.lattice = lat

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

class LeechFuzzyExtractor(FuzzyExtractor):
    def __init__(self, k):
        self.K = k
    
    # (helper) snap to the closest point in the
    # lattice, under K-step increments
    def closest(self, w):
        return Leech.decode(w, self.K)

    # (helper) hash a point/vector
    def hashVector(self, v):
        b = bytes(v)
        return sha256(b).hexdigest()

    # given an input w, generate the 
    # vector and snapped point
    # handles type conversion
    def gen(self, w):
        # format the input into a 24 integer vector
        # currently, it does no preprocessing
        return self.genFromVector(w)
    
    # given some point and a vector, 
    # find the closest lattice point
    # handles type conversion
    def recov(self, bs, w):
        # format w into a 24 integer vector
        # and pass it into recovFromVector
        # currently, it does no preprocessing
        return self.recovFromVector(bs, w)
    
    # helper for gen, works specifically
    # with vectors
    def genFromVector(self, w):
        ws = np.array(w)

        es = np.zeros(24, dtype=int)
        for i in range(24): es[i] = rdm.randint(1, 999)
        es = self.closest(es)

        bs = es - ws
        return bs, self.hashVector(es)

    # helper for recov, works specifically
    # with vectors
    def recovFromVector(self, bs, ws):
        e_prime = ws + bs
        e_prime = self.closest(e_prime)
        return self.hashVector(e_prime)

    