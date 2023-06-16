# %%
from torch_geometric.datasets import AMiner,OGB_MAG,DBLP,IMDB,LastFM,HGBDataset,Taobao
import torch_geometric.transforms as T
def load_graph(graph_name:str):
 
    if graph_name == 'aminer':
        data_set = AMiner(root='./data/aminer')
        graph = data_set[0]
        return graph
    elif graph_name == 'ogb-mag':
        data_set = OGB_MAG(root='./data/ogb_mag',transform=T.ToUndirected())
        graph = data_set[0]
        return graph
    elif graph_name == 'dblp':
        data_set = DBLP(root='./data/dblp')
        graph = data_set[0]
        return graph
    elif graph_name == 'imdb':
        data_set = IMDB(root='./data/imdb')
        graph = data_set[0]
        return graph
    elif graph_name == 'last-fm':
        data_set = LastFM(root='./data/last_fm')
        graph = data_set[0]
        return graph
    elif graph_name == 'hgb':
        data_set = HGBDataset(root='./data/hgb',name="ACM")
        graph = data_set[0]
        return graph
    elif graph_name == 'taobao':
        data_set = Taobao(root='./data/taobao',transform=T.ToUndirected())
        graph = data_set[0]
        return graph
    pass

# %%
if __name__ == "__main__":

    graph = load_graph('aminer')
    print("aminer")
    print(graph)
    graph = load_graph('ogb-mag')
    print("ogb-mag")
    print(graph)
    graph = load_graph('dblp')
    print("dblp")
    print(graph)
    graph = load_graph('imdb')
    print("imdb")
    print(graph)
    graph = load_graph('movie-lens')
    print("movie-lens")
    print(graph)
    graph = load_graph('last-fm')
    print("last-fm")
    print(graph)
    graph = load_graph('hgb')
    print("hgb")
    print(graph)
    graph = load_graph('taobao')
    print("taobao")
    print(graph)
# %%
