from pyFuzzImpl import FuzzyExtractor
import numpy as np

def makeHelpers(FE: FuzzyExtractor, A):
    keys, helpers = [], []

    for a in A:
        h, k = FE.gen(a)
        keys.append(k)
        helpers.append(h)
    
    return keys, helpers

def attemptMatching(FE: FuzzyExtractor, A_H, B): 
    #recov = np.zeros((len(A_H), len(B)))
    recov = [[None for _ in range(len(B))] for _ in range(len(A_H))]

    for i in range(len(B)):
        for j in range(len(A_H)):
            #print(FE.recov(A_H[j], B[i]))
            recov[j][i] = FE.recov(A_H[j], B[i])
    
    return recov

def returnMatches(R, A_K):
    out = []

    for i in range(len(R)):
        for j in range(len(R[i])):
            for k in range(len(A_K)):
                if np.array_equal(A_K[k], R[i][j]):
                    out.append(A_K[k])


    return out