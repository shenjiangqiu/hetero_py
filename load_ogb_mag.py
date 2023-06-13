# %%
def load_ogb_mag():
    from torch_geometric.datasets import OGB_MAG

    dataset = OGB_MAG(root='./data', preprocess='metapath2vec')
    data = dataset[0]
    import torch_geometric.transforms as T
    data_undirected = T.ToUndirected()(data)
    return data_undirected
# %%
if __name__ == "__main__":
    data = load_ogb_mag()
    print(data)
# %%
