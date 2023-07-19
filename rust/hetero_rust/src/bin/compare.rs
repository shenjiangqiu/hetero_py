use hashbrown::HashSet;
use hetero_rust::{
    load_hetero_graph, DblpNodeType, ImdbNodeType, LastFmNodeType, NodeId, OgbMagNodeType,
};
use std::{fmt::Debug, hash::Hash, hint::black_box, path::Path};

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
    //vec!["author", "paper", "field_of_study", "paper", "author"],
    let metapath = vec![
        OgbMagNodeType::Author,
        OgbMagNodeType::Paper,
        OgbMagNodeType::FieldOfStudy,
        OgbMagNodeType::Paper,
        OgbMagNodeType::Author,
    ];
    build_graph("splited_graph/ogb-mag", &metapath);
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

    // first setup the initial context
    let now = std::time::Instant::now();
    let context_design = hetero_rust::sim::design::build_context(&hetero_graph, meta_path);
    black_box(&context_design);

    println!(
        "design graph: {:?},build context: {:?}",
        graph_name,
        now.elapsed()
    );

    let now = std::time::Instant::now();

    let context_mecch = hetero_rust::sim::mecch::build_context(&hetero_graph, meta_path);
    black_box(&context_mecch);
    println!(
        " mecch graph: {:?},build context: {:?}",
        graph_name,
        now.elapsed()
    );
    if context_design != context_mecch {
        println!("context_design != context_mecch");
        show_difference(&context_design, &context_mecch);
    }
}

fn show_difference<NodeType: Eq + ToString + Hash + Debug>(
    context_1: &[HashSet<NodeId<NodeType>>],
    context_2: &[HashSet<NodeId<NodeType>>],
) {
    if context_1.len() != context_2.len() {
        println!("context_1.len()!=context_2.len()");
        return;
    }
    for (i, (c1, c2)) in context_1.iter().zip(context_2.iter()).enumerate() {
        if c1 != c2 {
            println!("context_1[{}]!=context_2[{}]", i, i);
            for c in c1 {
                if !c2.contains(c) {
                    println!("c1: {:?} not in c2", c);
                }
            }
            for c in c2 {
                if !c1.contains(c) {
                    println!("c2: {:?} not in c1", c);
                }
            }
        }
    }
}
