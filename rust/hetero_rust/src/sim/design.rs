use crate::{sim::BuildContextFn, HeteroGraph, NodeId};
use hashbrown::HashSet;
use std::hash::Hash;
pub struct DesignBuildContextFn;
impl BuildContextFn for DesignBuildContextFn {
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

pub fn build_context<NodeType: ToString + Eq + Hash + Clone + Copy>(
    hetero_graph: &HeteroGraph,
    metapath: &[NodeType],
) -> Vec<HashSet<NodeId<NodeType>>> {
    if metapath.len() == 2 {
        let sub_graph = hetero_graph
            .subgraphs
            .get(&(metapath[0].to_string(), metapath[1].to_string()))
            .unwrap();
        let mut context = Vec::with_capacity(sub_graph.rows());
        for (source_id, row_vec) in sub_graph.outer_iterator().enumerate() {
            let mut current_set = HashSet::new();

            for neighbor in row_vec.indices() {
                current_set.insert(NodeId {
                    layer: metapath[1],
                    id: *neighbor as u32,
                });
            }
            current_set.insert(NodeId {
                layer: metapath[0],
                id: source_id as u32,
            });
            context.push(current_set);
        }
        context
    } else {
        let sub_graph = hetero_graph
            .subgraphs
            .get(&(metapath[0].to_string(), metapath[1].to_string()))
            .unwrap();
        let mut context = Vec::with_capacity(sub_graph.rows());
        let previous_context = build_context(hetero_graph, &metapath[1..]);
        for (source_id, row_vec) in sub_graph.outer_iterator().enumerate() {
            let mut new_context = HashSet::new();

            for neighbor in row_vec.indices() {
                new_context.extend(
                    previous_context.get(*neighbor as usize).cloned().unwrap_or(
                        std::iter::once(NodeId {
                            layer: metapath[1],
                            id: *neighbor,
                        })
                        .collect(),
                    ),
                );
            }

            new_context.insert(NodeId {
                layer: metapath[0],
                id: source_id as u32,
            });
            context.push(new_context);
        }
        context
    }
}
