import sys
import numpy as np
import pandas as pd
from sklearn import metrics
import matplotlib.pyplot as plt

def load_data():
    with open("matches/pairs_with_euclid_1000_1.txt", "r") as file:
        lines = file.readlines()
    
    scale, tpr, fpr, et, ef = [], [], [], [], []
    data_map = {}
    
    for line in lines:
        val = list(map(float, line.split()))
        scale.append(val[0])
        data_map[val[0]] = (val[1], val[2], val[3], val[4])
    
    scale.sort()
    
    for a in scale:
        tpr.append(data_map[a][0])
        fpr.append(data_map[a][1])
        et.append(data_map[a][2])
        ef.append(data_map[a][3])
    
    return scale, tpr, fpr, et, ef

def compute_roc():
    scale, tpr, fpr, et, ef = load_data()
    # roc_auc = metrics.auc(fpr, tpr)
    
    plt.figure(figsize=(8, 6))
    # plt.plot(fpr, tpr, color='darkorange', lw=2, label=f'ROC curve (area = {roc_auc:.2f})')
    
    plt.plot(scale, tpr, 'g-', label='TPR')
    plt.plot(scale, fpr, 'r-', label='FPR')
    plt.plot(scale, et, 'g--', label='ET')
    plt.plot(scale, ef, 'r--', label='EF')
    
    # plt.plot([0, 1], [0, 1], color='navy', lw=2, linestyle='--')
    plt.xlim([0.0, 1.0])
    plt.ylim([0.0, 1.05])
    plt.xlabel('Scale')
    plt.ylabel('Matching')
    plt.title('Matching Stats')
    plt.legend(loc="lower right")
    plt.show()

if __name__ == "__main__":
    print(compute_roc())
