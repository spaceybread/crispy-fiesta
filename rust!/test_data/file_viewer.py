import numpy as np

data = np.load('embeddings.npy')

f =  open('embeddings.txt', 'w')

for vector in data:
    line = ', '.join(map(str, vector))
    f.write(line + '\n')
