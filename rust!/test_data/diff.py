import numpy as np

data = np.load('embeddings.npy')
id = open("files.txt", 'r')

mapp = {}
idx = 0
for line in id:
    filename = line.split("/")[9]
    mapp[idx] = filename
    idx += 1
        

idx = 0
for vector in data:
    line = ', '.join(map(str, vector))
    filename = mapp.get(idx)
    f =  open('imgs/' + filename + '.txt', 'a')
    f.write(line + '\n')
    idx += 1
