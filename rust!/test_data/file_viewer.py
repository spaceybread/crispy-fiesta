import numpy as np

data = np.load('embeddings.npy')

with open('embeddings.txt', 'w') as f:
    for vector in data:
        line = ', '.join(map(str, vector * 10))
        f.write(line + '\n')
