from hetro_py.metapath import Metapath
from hetro_py.load_graph import load_graph
import torch
dplp_metapath = Metapath("dblp",[
    ["author","paper","author"],
    ["author","paper","conference","paper","author"],
    ["author","paper","term","paper","author"]])

ibmd_metapath = Metapath("imbd",[
    ["movie","director","movie"],
    ["movie","actor","movie"],
    ["director","movie","actor","movie","director"],
    ["actor","movie","actor"],
    ["actor","movie","director","movie","actor"]])
lastfm_metapath = Metapath("lastfm",[
    ["user","artist","user"],
    ["user","artist","tag","artist","user"],
    ["artist","user","artist"],
    ["artist","tag","artist"]])
ogbmag_metapath = Metapath("ogbmag",[
    ["author","paper","author"],
    ["author","paper","field_of_study","paper","author"]])

def coo_to_scr(edge_index):
    coo=torch.sparse_coo_tensor(edge_index,torch.ones(edge_index.shape[1]))
    csr = coo.to_sparse_csr()
    return csr

def get_meta_count(g,meta_path:Metapath):
    for path in meta_path.metapath:
        count = len(path)
        print(count)
    pass

if __name__ ==   "__main__":
    g = load_graph("dblp")

    # get_meta_count(g,dplp_metapath)
    print(g)
    edge_index=g["paper","author"].edge_index
    print(edge_index)
    csr = coo_to_scr(edge_index)
    print(csr)
    pass


