import sys
import numpy as np
import pandas as pd
from sklearn import metrics
import matplotlib.pyplot as plt

def load_data():
    with open("matches/pairs.txt", "r") as file:
        lines = file.readlines()
    
    scale, tpr, fpr = [], [], []
    
    for line in lines:
        val = list(map(float, line.split()))
        scale.append(val[0])
        tpr.append(val[1])
        fpr.append(val[2])
    
    return scale, tpr, fpr

def compute_roc():
    scale, tpr, fpr = load_data()
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

if __name__ == "__main__":
    print(compute_roc())
