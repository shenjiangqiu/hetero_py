# %%
from hetro_py.load_graph import load_graph
import torch
import attrs2bin
import numpy as np
import matplotlib.pyplot as plt
from collections import Counter
def main():
    graphs = ["aminer","ogb-mag","dblp","imdb","last-fm","hgb","taobao"]
    for graph_name in graphs:
        print(graph_name)
        graph = load_graph(graph_name)
        nodes,edges = graph.metadata()
        for edge in edges:
            print(edge)
            bincount=np.bincount( graph[edge].edge_index[0])
            histo = Counter(bincount)
            print(histo)
            # 绘制直方图
            plt.bar(histo.keys(), histo.values())
            plt.xlabel('Node Degree')
            plt.ylabel('Node Count')
            plt.title('Histogram of Node Degree'+'_'.join(edge))
            plt.show()


main()
# %%
