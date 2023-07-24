use itertools::Itertools;
use rayon::prelude::*;
use sim::NodeTypeFeatureSize;
use sprs::{num_kinds::Pattern, CsMatI, CsVecI, TriMatI};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    hash::Hash,
    io::BufReader,
    path::{Path, PathBuf},
    sync::{atomic::AtomicUsize, RwLock},
};
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

pub mod sim;

pub fn init_log(filter: LevelFilter) {
    let filter = EnvFilter::builder()
        .with_default_directive(filter.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .unwrap_or_default();
}
pub fn init_log_info() {
    init_log(LevelFilter::INFO);
}
pub fn init_log_error() {
    init_log(LevelFilter::ERROR);
}

/// load a single graph from a mtx file
fn load_graph(graph_path: &Path) -> eyre::Result<CsMatI<Pattern, u32>> {
    let mut file = BufReader::new(File::open(graph_path)?);
    let graph: TriMatI<Pattern, u32> = sprs::io::read_matrix_market_from_bufread(&mut file)?;
    Ok(graph.to_csr())
}

/// load the hetero graph from a dir, the mtx file name should be like "edge_type1$edge_type2.mtx"
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

#[derive(Debug, Clone)]
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
    pub subgraphs: HashMap<(String, String), CsMatI<Pattern, u32>>,
}
pub struct TaskPack {
    pub graph_name: String,
    pub graph_path: PathBuf,
    pub metapathes: MetaPathesPack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DblpNodeType {
    Author,
    Paper,
    Conference,
    Term,
}
impl ToString for DblpNodeType {
    fn to_string(&self) -> String {
        match self {
            DblpNodeType::Author => "author".to_string(),
            DblpNodeType::Paper => "paper".to_string(),
            DblpNodeType::Conference => "conference".to_string(),
            DblpNodeType::Term => "term".to_string(),
        }
    }
}
impl sim::NodeTypeFeatureSize for DblpNodeType {
    fn feature_size(&self) -> usize {
        128
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImdbNodeType {
    Movie,
    Director,
    Actor,
}
impl ToString for ImdbNodeType {
    fn to_string(&self) -> String {
        match self {
            ImdbNodeType::Movie => "movie".to_string(),
            ImdbNodeType::Director => "director".to_string(),
            ImdbNodeType::Actor => "actor".to_string(),
        }
    }
}
impl NodeTypeFeatureSize for ImdbNodeType {
    fn feature_size(&self) -> usize {
        128
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LastFmNodeType {
    User,
    Artist,
    Tag,
}
impl ToString for LastFmNodeType {
    fn to_string(&self) -> String {
        match self {
            LastFmNodeType::User => "user".to_string(),
            LastFmNodeType::Artist => "artist".to_string(),
            LastFmNodeType::Tag => "tag".to_string(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OgbMagNodeType {
    Author,
    Institution,
    FieldOfStudy,
    Paper,
}
impl ToString for OgbMagNodeType {
    fn to_string(&self) -> String {
        match self {
            OgbMagNodeType::Author => "author".to_string(),
            OgbMagNodeType::Institution => "institution".to_string(),
            OgbMagNodeType::FieldOfStudy => "field_of_study".to_string(),
            OgbMagNodeType::Paper => "paper".to_string(),
        }
    }
}
impl NodeTypeFeatureSize for LastFmNodeType {
    fn feature_size(&self) -> usize {
        128
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId<NodeType> {
    pub layer: NodeType,
    pub id: u32,
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

/// count the number of metapathes
pub fn count_metapath_memory_usage(graph: &HeteroGraph, metapath: &Metapath) -> usize {
    let first_edge_type = (metapath.path[0].clone(), metapath.path[1].clone());
    let first_graph = graph.subgraphs.get(&first_edge_type).unwrap();
    let rows = first_graph.rows();
    let count_cache = RwLock::new(BTreeMap::new());
    let finished_tasks = AtomicUsize::new(0);
    let count = (0..rows)
        .into_par_iter()
        .map(|i| {
            let count = count_partial_metapath_count(graph, &metapath.path[0..], i, &count_cache);
            finished_tasks.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let finished_tasks = finished_tasks.load(std::sync::atomic::Ordering::Relaxed);
            if finished_tasks % 10000 == 0 {
                tracing::info!(?metapath, "finished tasks: {}/{}", finished_tasks, rows);
            }

            count
        })
        .sum::<usize>();
    count * 4
}

fn count_partial_metapath_count(
    graph: &HeteroGraph,
    partial_metapath: &[String],
    start_node: usize,
    count_cache: &RwLock<BTreeMap<(usize, usize), usize>>,
) -> usize {
    let current_count = count_cache.read().unwrap();
    if let Some(count) = current_count.get(&(start_node, partial_metapath.len())) {
        return *count;
    }
    drop(current_count);

    if partial_metapath.len() == 2 {
        let edge_type = (partial_metapath[0].clone(), partial_metapath[1].clone());

        let sub_graph = graph.subgraphs.get(&edge_type).unwrap();

        let neighbers = sub_graph.outer_view(start_node).unwrap();
        tracing::debug!(start_node, "final neighbers: {:?}", neighbers.nnz());
        let count = neighbers.nnz();
        count_cache
            .write()
            .unwrap()
            .insert((start_node, partial_metapath.len()), count);
        count
    } else {
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
        let count = neighbers
            .indices()
            .into_iter()
            .map(|col| {
                tracing::debug!(start_node, "neighbers: {:?}", col);
                count_partial_metapath_count(
                    graph,
                    &partial_metapath[1..],
                    *col as usize,
                    count_cache,
                )
            })
            .sum();
        let meta_len = partial_metapath.len();
        tracing::debug!(start_node, meta_len, "temp neighbers: {:?}", count);

        let mut current_count = count_cache.write().unwrap();
        current_count.insert((start_node, meta_len), count);

        count
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::Path, sync::RwLock};

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
        let count_cache = RwLock::new(BTreeMap::new());
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
            &count_cache,
        );
        println!("meta_path: {:?}", meta_path);
    }
}
