use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use rayon::prelude::*;
use sprs::{num_kinds::Pattern, CsMatI, CsVecI, TriMatI};

pub fn load_graph(graph_path: &Path) -> eyre::Result<CsMatI<Pattern, u32>> {
    let mut file = BufReader::new(File::open(graph_path)?);
    let graph: TriMatI<Pattern, u32> = sprs::io::read_matrix_market_from_bufread(&mut file)?;
    Ok(graph.to_csr())
}

pub fn load_hetero_graph(graph_path: impl AsRef<Path>) -> eyre::Result<HeteroGraph> {
    // first read the dir, get all mtx files
    let mut subgraphs = HashMap::new();
    for entry in std::fs::read_dir(graph_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let edge_type: (String, String) = path
                .file_stem()
                .ok_or(eyre::eyre!("no term for this file"))?
                .to_str()
                .ok_or(eyre::eyre!("cannot build str"))?
                .split('$')
                .map(ToString::to_string)
                .collect_tuple()
                .ok_or(eyre::eyre!("bad file name"))?;
            let graph = load_graph(&path)?;
            subgraphs.insert(edge_type, graph);
        }
    }
    Ok(HeteroGraph { subgraphs })
}
#[derive(Debug)]
pub struct Metapath {
    path: Vec<String>,
}

#[derive(Debug)]
pub struct MetaPathesPack {
    pub metapathes: Vec<Metapath>,
}

impl<T, I> From<I> for Metapath
where
    T: Into<String>,
    I: IntoIterator<Item = T>,
{
    fn from(path: I) -> Self {
        let path = path.into_iter().map(Into::into).collect();
        Self { path }
    }
}
impl<T, I> From<I> for MetaPathesPack
where
    T: Into<Metapath>,
    I: IntoIterator<Item = T>,
{
    fn from(path: I) -> Self {
        let metapathes = path.into_iter().map(Into::into).collect();
        Self { metapathes }
    }
}

#[derive(Debug)]
pub struct HeteroGraph {
    subgraphs: HashMap<(String, String), CsMatI<Pattern, u32>>,
}
pub struct TaskPack {
    pub graph_name: String,
    pub graph_path: PathBuf,
    pub metapathes: MetaPathesPack,
}

pub fn get_all_taskpack() -> Vec<TaskPack> {
    let graphs = ["dblp", "imdb", "last-fm", "ogb-mag"];
    let metapathes = [
        vec![
            vec!["author", "paper", "author"],
            vec!["author", "paper", "conference", "paper", "author"],
            vec!["author", "paper", "term", "paper", "author"],
        ],
        vec![
            vec!["movie", "director", "movie"],
            vec!["movie", "actor", "movie"],
            vec!["director", "movie", "actor", "movie", "director"],
            vec!["actor", "movie", "actor"],
            vec!["actor", "movie", "director", "movie", "actor"],
        ],
        vec![
            vec!["user", "artist", "user"],
            vec!["user", "artist", "tag", "artist", "user"],
            vec!["artist", "user", "artist"],
            vec!["artist", "tag", "artist"],
        ],
        vec![
            vec!["author", "paper", "author"],
            vec!["author", "paper", "field_of_study", "paper", "author"],
        ],
    ];
    graphs
        .into_iter()
        .zip(metapathes)
        .map(|(graph_name, metapath)| {
            let graph_path = format!("splited_graph/{}", graph_name);
            let graph_path = PathBuf::from(graph_path);
            let metapathes = metapath.into();
            TaskPack {
                graph_name: graph_name.to_string(),
                graph_path,
                metapathes,
            }
        })
        .collect()
}

pub fn count_metapath_memory_usage(graph: &HeteroGraph, metapath: &Metapath) -> usize {
    let first_edge_type = (metapath.path[0].clone(), metapath.path[1].clone());
    let first_graph = graph.subgraphs.get(&first_edge_type).unwrap();
    let rows = first_graph.rows();
    let count = (0..rows)
        .into_par_iter()
        .map(|i| count_partial_metapath_count(graph, &metapath.path[0..], i))
        .sum::<usize>();
    count * 4
}
pub fn count_partial_metapath_count(
    graph: &HeteroGraph,
    partial_metapath: &[String],
    start_node: usize,
) -> usize {
    if partial_metapath.len() == 2 {
        let edge_type = (partial_metapath[0].clone(), partial_metapath[1].clone());

        let sub_graph = graph.subgraphs.get(&edge_type).unwrap();

        let neighbers = sub_graph.outer_view(start_node).unwrap();
        tracing::debug!(start_node, "final neighbers: {:?}", neighbers.nnz());
        neighbers.nnz()
    } else {
        assert!(partial_metapath.len() > 2);
        let edge_type = (partial_metapath[0].clone(), partial_metapath[1].clone());

        let sub_graph = graph.subgraphs.get(&edge_type).unwrap();
        let empty = CsVecI::empty(0);
        let neighbers = sub_graph.outer_view(start_node).unwrap_or_else(|| {
            tracing::error!(
                "cannot get neighbers for node {} with edge type {:?}",
                start_node,
                edge_type,
            );
            empty.view()
        });
        let mut count = 0;
        for col in neighbers.indices() {
            tracing::debug!(start_node, "neighbers: {:?}", col);
            count += count_partial_metapath_count(graph, &partial_metapath[1..], *col as usize);
        }
        let meta_len = partial_metapath.len();
        tracing::debug!(start_node, meta_len, "temp neighbers: {:?}", count);
        count
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tracing_subscriber::EnvFilter;

    use crate::{count_partial_metapath_count, load_graph, load_hetero_graph};

    #[test]
    fn test_load_dblp() {
        let graph = load_graph(Path::new(
            "/home/sjq/git/hetero_py/splited_graph/dblp/author_paper.mtx",
        ))
        .unwrap();
        println!("cols: {:?}", graph.cols());
        println!("rows: {:?}", graph.rows());
    }
    #[test]
    fn test_load_heterograph() {
        let hetero_graph =
            load_hetero_graph(Path::new("/home/sjq/git/hetero_py/splited_graph/dblp")).unwrap();
        println!("hetero_graph: {:?}", hetero_graph);
    }
    #[test]
    fn test_count() {
        let filter = EnvFilter::new("debug");
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .try_init()
            .unwrap_or(());
        let graph =
            load_hetero_graph(Path::new("/home/sjq/git/hetero_py/splited_graph/dblp")).unwrap();
        let meta_path = count_partial_metapath_count(
            &graph,
            &[
                "author".to_string(),
                "paper".to_string(),
                "conference".to_string(),
                "paper".to_string(),
                "author".to_string(),
            ],
            0,
        );
        println!("meta_path: {:?}", meta_path);
    }
}
