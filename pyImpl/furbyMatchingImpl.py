from pyFuzzImpl import FuzzyExtractor
import numpy as np

class FuzzyMatching:
    # constructor for FSM, simply accepts 
    # a Fuzzy Extractor object
    def __init__(self, FE: FuzzyExtractor):
        self.FE = FE

    # for every element in A, compute the 
    # key-helper pair and save it, in order
    def makeHelpers(self, A):
        keys, helpers = [], []

        for a in A:
            h, k = self.FE.gen(a)
            keys.append(k)
            helpers.append(h)
    
        return keys, helpers

    # take the helpers and the *other* set and
    # compute the resulting keys for all pairs
    def attemptMatching(self, A_H, B): 
        #recov = np.zeros((len(A_H), len(B)))
        recov = [[None for _ in range(len(B))] for _ in range(len(A_H))]
        # this doesn't seem to like working with
        # the numpy 2D matrix init so I have kept the
        # old def for the matrix here

        for i in range(len(B)):
            for j in range(len(A_H)):
                recov[j][i] = self.FE.recov(A_H[j], B[i])
    
        return recov

    # with all possible recovered pairs and the
    # original list of keys, do a pair wise check
    # and find matches
    def returnMatches(self, R, A, A_K):
        out = []

        for i in range(len(R)):
            for j in range(len(R[i])):
                if np.array_equal(A_K[i], R[i][j]):
                    out.append(A[i])
                    

        # set to remove duplicates
        # sort so that it looks nice
        out = list(set(out))
        return sorted(out)
    
    # with all possible recovered pairs and the
    # original list of keys, do a pair wise check
    # and find matches (compatible with hashes)
    def returnMatchesAsHashes(self, R, A, A_K):
        out = []

        for i in range(len(R)):
            for j in range(len(R[i])):
                if A_K[i] == R[i][j]:
                    out.append(A[i])
                    

        # set to remove duplicates
        # sort so that it looks nice
        return out
    
    # debug function that works almost exactly
    # the same as the function above but returns
    # the matched pairs
    def returnMatchesAsPairs(self, R, A, A_K, B):
        out = []

        for i in range(len(R)):
            for j in range(len(R[i])):
                if np.array_equal(A_K[i], R[i][j]):
                    out.append((A[i], B[j]))
                    
        # sort so that it looks nice
        return sorted(out)
