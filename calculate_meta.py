from hetro_py.metapath import Metapath
from hetro_py.load_graph import load_graph
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


def get_meta_count(g,meta_path:Metapath):
    for path in meta_path.metapath:
        count = len(path)
        print(count)
    pass

if __name__ ==   "__main__":
    g = load_graph("dblp")
    get_meta_count(g,dplp_metapath)
    pass


