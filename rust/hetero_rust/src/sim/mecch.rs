use std::hash::Hash;

use crate::{sim::BuildContextFn, HeteroGraph, NodeId};
use hashbrown::HashSet;
pub struct MecchBuildContextFn;
impl BuildContextFn for MecchBuildContextFn {
    fn build_context<NodeType>(
        graph: &HeteroGraph,
        metapath: &[NodeType],
    ) -> Vec<HashSet<NodeId<NodeType>>>
    where
        NodeType: ToString + Eq + Hash + Clone + Copy,
    {
        build_context(graph, metapath)
    }
}

pub fn build_context<NodeType>(
    hetero_graph: &HeteroGraph,
    metapath: &[NodeType],
) -> Vec<HashSet<NodeId<NodeType>>>
where
    NodeType: ToString + Eq + Hash + Clone + Copy,
{
    let sub_graph = hetero_graph
        .subgraphs
        .get(&(metapath[0].to_string(), metapath[1].to_string()))
        .unwrap();
    let mut context = Vec::with_capacity(sub_graph.rows());
    for i in 0..sub_graph.rows() {
        let mut current_set = HashSet::new();
        build_context_for_single_node(hetero_graph, metapath, &mut current_set, i);
        context.push(current_set);
        tracing::debug!("finished node {}/{}", i, sub_graph.rows());
    }
    context
}
fn build_context_for_single_node<NodeType: ToString + Eq + Hash + Clone + Copy>(
    hetero_graph: &HeteroGraph,
    metapath: &[NodeType],
    current_set: &mut HashSet<NodeId<NodeType>>,
    current_node: usize,
) {
    if metapath.len() == 2 {
        let sub_graph = hetero_graph
            .subgraphs
            .get(&(metapath[0].to_string(), metapath[1].to_string()))
            .unwrap();
        for neighbor in sub_graph.outer_view(current_node).unwrap().indices() {
            current_set.insert(NodeId {
                layer: metapath[1],
                id: *neighbor as u32,
            });
        }
        current_set.insert(NodeId {
            layer: metapath[0],
            id: current_node as u32,
        });
    } else {
        let sub_graph = hetero_graph
            .subgraphs
            .get(&(metapath[0].to_string(), metapath[1].to_string()))
            .unwrap();
        if let Some(node_vec) = sub_graph.outer_view(current_node) {
            for neighbor in node_vec.indices() {
                build_context_for_single_node(
                    hetero_graph,
                    &metapath[1..],
                    current_set,
                    *neighbor as usize,
                );
            }
        }
        current_set.insert(NodeId {
            layer: metapath[0],
            id: current_node as u32,
        });
    }
}
