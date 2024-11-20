import sys
import numpy as np
import pandas as pd
from sklearn import metrics
import matplotlib.pyplot as plt

MIN_IMG_TH = 4
def _load_data(npz_file, info_file):
    a = np.load(npz_file)
    f = [x.strip().split('/')[-2:] for x in open(info_file)]
    d = pd.DataFrame(a)
    d[['name', 'imgid']] = f
    ids = d.groupby('name').apply(lambda x: list(x.index))
    ids = ids[ids.apply(lambda x: len(x) >= MIN_IMG_TH)]
    print(f"data loaded. {d.shape} entries. {ids.max()}")
    return d, ids

def _sample(a, n, t):
    return np.hstack(a.sample(n).apply(
        lambda x: np.random.choice(x, t, replace=False)
    ))

def _euclidian_dist(a, b):
    return ((a-b)**2).sum(axis=1)

def _dot_dist(a, b):
    a = (a-a.mean())/a.std()
    b = (b-b.mean())/b.std()
    return (a*b).sum(axis=1)


def compute_roc(npz_file, file_name):
    d, ids = _load_data(npz_file, file_name)
    dim = d.shape[1]-2
    n = 100
    intra_pairs = _sample(ids, n, 2).reshape((n, 2))
    inter_pairs = _sample(ids, 2*n, 1).reshape((n, 2))
    
    t = np.vstack((intra_pairs, inter_pairs))
    v1 = d.iloc[t[:, 0], np.arange(dim)].values
    v2 = d.iloc[t[:, 1], np.arange(dim)].values
    y = [0] * n + [1] * n
    scores1 = _euclidian_dist(v1, v2)
    scores2 = _dot_dist(v1, v2)
    fpr, tpr, thresholds = metrics.roc_curve(y, scores2)
    roc_auc = metrics.auc(fpr, tpr)

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
    return fpr, tpr, thresholds




    
if __name__ == "__main__":
    print(compute_roc(sys.argv[1], sys.argv[2]))
