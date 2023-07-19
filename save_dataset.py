# %%
import asyncio
from hetro_py.load_graph import load_graph
from torch_geometric.data import HeteroData
import os


def write_single_edge_type(f, edge_index):
    # get the max node index in start node
    max_start_node = edge_index[0].max().item()+1
    # get the max node index in end node
    max_end_node = edge_index[1].max().item()+1
    # write the mtx header, the data type is pattern
    f.write('%%MatrixMarket matrix coordinate pattern general\n')
    f.write(f'{max_start_node} {max_end_node} {edge_index.shape[1]}\n')
    # write the mtx data
    for i in range(edge_index.shape[1]):
        f.write(f'{edge_index[0][i].item()+1} {edge_index[1][i].item()+1}\n')


def save_dataset(name: str, data: HeteroData):
    node_types, edge_types = data.metadata()
    for edge_type in edge_types:
        start_node = edge_type[0]
        end_node = edge_type[2]
        print(edge_type)
        edge_index = data[edge_type].edge_index
        print(edge_index)
        # write the mtx file
        file_name = f'splited_graph/{name}/{start_node}${end_node}.mtx'
        # make dir
        os.makedirs(os.path.dirname(file_name), exist_ok=True)

        with open(file_name, 'w') as f:
            write_single_edge_type(f, edge_index)


# %%
graph_names = ['aminer', 'ogb-mag', 'dblp', 'imdb', 'last-fm', 'hgb', 'taobao']


def background(f):
    def wrapped(*args, **kwargs):
        return asyncio.get_event_loop().run_in_executor(None, f, *args, **kwargs)

    return wrapped


def save_graph(graph_name):
    graph = load_graph(graph_name)
    save_dataset(graph_name, graph)
    del graph


save_graph("ogb-mag")

# %%
