use hashbrown::HashMap;
use hetero_rust::{load_hetero_graph, DblpNodeType, ImdbNodeType, LastFmNodeType};
use itertools::Itertools;
use rand::Rng;
use std::{
    fmt::Debug,
    hash::Hash,
    hint::black_box,
    ops::{AddAssign, SubAssign},
    path::Path,
};

fn main() {
    let metapath = vec![
        DblpNodeType::Author,
        DblpNodeType::Paper,
        DblpNodeType::Conference,
        DblpNodeType::Paper,
        DblpNodeType::Author,
    ];
    build_graph("splited_graph/dblp", &metapath);

    // let metapath = vec!["director", "movie", "actor", "movie", "director"];
    let metapath = vec![
        ImdbNodeType::Director,
        ImdbNodeType::Movie,
        ImdbNodeType::Actor,
        ImdbNodeType::Movie,
        ImdbNodeType::Director,
    ];
    build_graph("splited_graph/imdb", &metapath);

    // let metapath = vec!["user", "artist", "tag", "artist", "user"];
    let metapath = vec![
        LastFmNodeType::User,
        LastFmNodeType::Artist,
        LastFmNodeType::Tag,
        LastFmNodeType::Artist,
        LastFmNodeType::User,
    ];
    build_graph("splited_graph/last-fm", &metapath);
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct MyFeature([i32; 128]);
impl AddAssign for MyFeature {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.0.len() {
            self.0[i] += rhs.0[i];
        }
    }
}
impl SubAssign for MyFeature {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.0.len() {
            self.0[i] -= rhs.0[i];
        }
    }
}
impl Default for MyFeature {
    fn default() -> Self {
        Self([0; 128])
    }
}

fn build_graph<NodeType: ToString + Eq + Hash + Clone + Copy + Debug>(
    graph: impl AsRef<Path>,
    meta_path: &[NodeType],
) {
    let graph_name = graph
        .as_ref()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let hetero_graph = load_hetero_graph(graph).unwrap();
    let node_rows: HashMap<NodeType, usize> = get_node_rows(&hetero_graph, meta_path);
    // first setup the initial context
    let mut node_features_store = HashMap::new();
    let mut rng = rand::thread_rng();
    for node_type in meta_path {
        if node_features_store.contains_key(node_type) {
            continue;
        }
        let mut all_nodes_feat = Vec::with_capacity(node_rows[node_type]);
        for _i in 0..node_rows[node_type] {
            let mut array_vec: arrayvec::ArrayVec<i32, 128> = arrayvec::ArrayVec::new();
            for _ in 0..array_vec.capacity() {
                array_vec.push(rng.gen());
            }
            let node_feat: [i32; 128] = array_vec.into_inner().unwrap();
            all_nodes_feat.push(MyFeature(node_feat));
        }

        node_features_store.insert(*node_type, all_nodes_feat);
    }
    let node_features = node_features_store
        .iter()
        .map(|(k, v)| (*k, v.as_slice()))
        .collect::<HashMap<_, _>>();

    let now = std::time::Instant::now();

    let (_context_design, result_design) =
        hetero_rust::sim::design_agg::build_context_and_aggregation(
            &hetero_graph,
            meta_path,
            &node_features,
        );
    black_box(&result_design);

    println!(
        "design graph: {:?},build context: {:?}",
        graph_name,
        now.elapsed()
    );

    let now = std::time::Instant::now();

    let result_mecch =
        hetero_rust::sim::mecch_agg::build_context_agg(&hetero_graph, meta_path, &node_features);
    black_box(&result_mecch);
    println!(
        " mecch graph: {:?},build context: {:?}",
        graph_name,
        now.elapsed()
    );
    if result_design != result_mecch {
        println!("context_design != context_mecch");
    }
}

fn get_node_rows<NodeType: ToString + Eq + Hash + Clone + Copy + Debug>(
    hetero_graph: &hetero_rust::HeteroGraph,
    meta_path: &[NodeType],
) -> HashMap<NodeType, usize> {
    let mut node_rows = HashMap::new();
    for (node_1, node_2) in meta_path.into_iter().tuple_windows() {
        if node_rows.contains_key(node_1) {
            continue;
        }
        let node_1_str = node_1.to_string();
        let node_2_str = node_2.to_string();
        let node_1_rows = hetero_graph
            .subgraphs
            .get(&(node_1_str.clone(), node_2_str.clone()))
            .unwrap();

        node_rows.insert(*node_1, node_1_rows.rows());
    }
    node_rows
}
