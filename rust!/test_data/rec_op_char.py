from sklearn.metrics import roc_curve, auc, pairwise
import matplotlib.pyplot as plt
from scipy import sparse
import numpy as np
import random

N = 50000

def make_data(n):
    vectors = np.load('embeddings.npy')
    with open('files.txt', 'r') as f:
        keys = [line.split("/")[9] for line in f.readlines()]

    hm = {key: vector for key, vector in zip(keys, vectors)}
    
    vector_a = []
    vector_b = []
    ground_truth = []

    for i in range(n):
    
        # adding a true pair
        k = random.choice(list(hm.keys()))
        
        a = random.choice(hm[k])
        b = a
        while b != a: b = random.choice(hm[k])
        
        vector_a.append(a)
        vector_b.append(b)
        ground_truth.append(True)
        
        # adding a false pair
        k = random.choice(list(hm.keys()))
        g = k
        while g != k: g = random.choice(list(hm.keys()))
        
        vector_a.append(random.choice(hm[k]))
        vector_b.append(random.choice(hm[g]))
        ground_truth.append(False)
    
    return vector_a, vector_b, ground_truth

vector_a, vector_b, ground_truth = make_data(N // 2)

scores = [np.linalg.norm(-vector_a[i] + vector_b[i]) for i in range(N)]

fpr, tpr, thresholds = roc_curve(ground_truth, scores)
roc_auc = auc(fpr, tpr)

plt.figure(figsize=(8, 6))
plt.plot(fpr, tpr, color='darkorange', lw=2, label=f'ROC curve (area = {roc_auc:.2f})')
plt.plot([0, 1], [0, 1], color='navy', lw=2, linestyle='--')
plt.xlim([0.0, 1.0])
plt.ylim([0.0, 1.05])
plt.xlabel('False Positive Rate')
plt.ylabel('True Positive Rate')
plt.title('Receiver Operating Characteristic')
plt.legend(loc="lower right")
plt.show()

