//! this module define the traits of the simulation

use crate::{DblpNodeType, HeteroGraph, ImdbNodeType, LastFmNodeType, NodeId, OgbMagNodeType};
use hashbrown::{HashMap, HashSet};
use std::{
    hash::Hash,
    hint::black_box,
    ops::{AddAssign, SubAssign},
    path::Path,
};
pub mod design;
pub mod design_agg;
pub mod mecch;
pub mod mecch_agg;
/// the feature size of the node type
pub trait NodeTypeFeatureSize {
    fn feature_size(&self) -> usize {
        128
    }
}

/// for each implementation, it should have a build context Fn
pub trait BuildContextFn {
    /// build the context for the metapath
    fn build_context<NodeType>(
        graph: &HeteroGraph,
        metapath: &[NodeType],
    ) -> Vec<HashSet<NodeId<NodeType>>>
    where
        NodeType: ToString + Eq + Hash + Clone + Copy;
}

/// build context and aggregation, describe an algorithm
pub trait BuildContextFnAgg {
    /// build the context for the metapath
    fn build_context_agg<ResultType, NodeType>(
        graph: &HeteroGraph,
        metapath: &[NodeType],
        node_features: &HashMap<NodeType, &[ResultType]>,
    ) -> Vec<ResultType>
    where
        NodeType: ToString + Eq + Hash + Clone + Copy,
        ResultType: AddAssign + SubAssign + Default + Clone + Copy;
}

/// given a graph and a metapath, build the context and test the time cost
/// - `build_context_fn` is the function to build the context
pub fn test_graph_with_metapath<NodeType>(
    graph: impl AsRef<Path>,
    meta_path: &[NodeType],
    build_context_fn: impl FnOnce(&HeteroGraph, &[NodeType]) -> Vec<HashSet<NodeId<NodeType>>>,
) where
    NodeType: ToString + Eq + Hash + Clone + Copy,
{
    let graph_name = graph
        .as_ref()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let hetero_graph = crate::load_hetero_graph(graph).unwrap();

    // first setup the initial context
    let now = std::time::Instant::now();
    let context = build_context_fn(&hetero_graph, meta_path);
    black_box(context);
    println!("graph: {:?},build context: {:?}", graph_name, now.elapsed());
}

/// given a graph and a metapath, build the context and test the time cost
/// - `build_context_fn` is the function to build the context
pub fn test_graph_with_metapath_compare<NodeType>(
    graph: impl AsRef<Path>,
    meta_path: &[NodeType],
    build_context_fn_1: impl FnOnce(&HeteroGraph, &[NodeType]) -> Vec<HashSet<NodeId<NodeType>>>,
    build_context_fn_2: impl FnOnce(&HeteroGraph, &[NodeType]) -> Vec<HashSet<NodeId<NodeType>>>,
) where
    NodeType: ToString + Eq + Hash + Clone + Copy,
{
    let graph_name = graph
        .as_ref()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let hetero_graph = crate::load_hetero_graph(graph).unwrap();

    // first setup the initial context
    let now = std::time::Instant::now();
    let context = build_context_fn_1(&hetero_graph, meta_path);
    black_box(context);
    println!(
        "graph: for method 1: {:?},build context: {:?}",
        graph_name,
        now.elapsed()
    );
    let now = std::time::Instant::now();
    let context = build_context_fn_2(&hetero_graph, meta_path);
    black_box(context);
    println!(
        "graph: for method 2: {:?},build context: {:?}",
        graph_name,
        now.elapsed()
    );
}

/// test the build context fn for all graph
pub fn main_test<ContextFn: BuildContextFn>() {
    let metapath = vec![
        DblpNodeType::Author,
        DblpNodeType::Paper,
        DblpNodeType::Conference,
        DblpNodeType::Paper,
        DblpNodeType::Author,
    ];
    test_graph_with_metapath("splited_graph/dblp", &metapath, ContextFn::build_context);

    // let metapath = vec!["director", "movie", "actor", "movie", "director"];
    let metapath = vec![
        ImdbNodeType::Director,
        ImdbNodeType::Movie,
        ImdbNodeType::Actor,
        ImdbNodeType::Movie,
        ImdbNodeType::Director,
    ];
    test_graph_with_metapath("splited_graph/imdb", &metapath, ContextFn::build_context);

    // let metapath = vec!["user", "artist", "tag", "artist", "user"];
    let metapath = vec![
        LastFmNodeType::User,
        LastFmNodeType::Artist,
        LastFmNodeType::Tag,
        LastFmNodeType::Artist,
        LastFmNodeType::User,
    ];
    test_graph_with_metapath("splited_graph/last-fm", &metapath, ContextFn::build_context);

    let metapath = vec![
        OgbMagNodeType::Author,
        OgbMagNodeType::Paper,
        OgbMagNodeType::FieldOfStudy,
        OgbMagNodeType::Paper,
        OgbMagNodeType::Author,
    ];
    test_graph_with_metapath("splited_graph/ogb-mag", &metapath, ContextFn::build_context);
}

pub fn main_test_compare<ContextFn1: BuildContextFn, ContextFn2: BuildContextFn>() {
    let metapath = vec![
        DblpNodeType::Author,
        DblpNodeType::Paper,
        DblpNodeType::Conference,
        DblpNodeType::Paper,
        DblpNodeType::Author,
    ];
    test_graph_with_metapath_compare(
        "splited_graph/dblp",
        &metapath,
        ContextFn1::build_context,
        ContextFn2::build_context,
    );

    // let metapath = vec!["director", "movie", "actor", "movie", "director"];
    let metapath = vec![
        ImdbNodeType::Director,
        ImdbNodeType::Movie,
        ImdbNodeType::Actor,
        ImdbNodeType::Movie,
        ImdbNodeType::Director,
    ];
    test_graph_with_metapath_compare(
        "splited_graph/imdb",
        &metapath,
        ContextFn1::build_context,
        ContextFn2::build_context,
    );

    // let metapath = vec!["user", "artist", "tag", "artist", "user"];
    let metapath = vec![
        LastFmNodeType::User,
        LastFmNodeType::Artist,
        LastFmNodeType::Tag,
        LastFmNodeType::Artist,
        LastFmNodeType::User,
    ];
    test_graph_with_metapath_compare(
        "splited_graph/last-fm",
        &metapath,
        ContextFn1::build_context,
        ContextFn2::build_context,
    );

    let metapath = vec![
        OgbMagNodeType::Author,
        OgbMagNodeType::Paper,
        OgbMagNodeType::FieldOfStudy,
        OgbMagNodeType::Paper,
        OgbMagNodeType::Author,
    ];
    test_graph_with_metapath_compare(
        "splited_graph/ogb-mag",
        &metapath,
        ContextFn1::build_context,
        ContextFn2::build_context,
    );
}
