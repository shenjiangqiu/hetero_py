//! the simulation include context build and aggregation
//!
use crate::{HeteroGraph, NodeId};
use hashbrown::{HashMap, HashSet};
use std::{
    hash::Hash,
    ops::{AddAssign, SubAssign},
};
use tracing::debug;

use super::BuildContextFnAgg;
pub struct DesignBuildContextAggFn;
impl BuildContextFnAgg for DesignBuildContextAggFn {
    fn build_context_agg<ResultType, NodeType>(
        graph: &HeteroGraph,
        metapath: &[NodeType],
        node_features: &HashMap<NodeType, &[ResultType]>,
    ) -> Vec<ResultType>
    where
        NodeType: ToString + Eq + Hash + Clone + Copy,
        ResultType: AddAssign + SubAssign + Default + Clone + Copy,
    {
        let result = build_context_and_aggregation(graph, metapath, node_features);
        result.1
    }
}

///
///
pub fn build_context_and_aggregation<NodeType, ResultType>(
    hetero_graph: &HeteroGraph,
    metapath: &[NodeType],
    node_features: &HashMap<NodeType, &[ResultType]>,
) -> (Vec<HashSet<NodeId<NodeType>>>, Vec<ResultType>)
where
    NodeType: ToString + Eq + Hash + Clone + Copy,
    ResultType: AddAssign + SubAssign + Default + Clone + Copy,
{
    if metapath.len() == 2 {
        // the last two layer
        debug!("building the last two layer");
        let sub_graph = hetero_graph
            .subgraphs
            .get(&(metapath[0].to_string(), metapath[1].to_string()))
            .unwrap();
        let mut context = Vec::with_capacity(sub_graph.rows());
        let mut results = Vec::with_capacity(sub_graph.rows());
        for (source_id, row_vec) in sub_graph.outer_iterator().enumerate() {
            if source_id % 1000 == 0 {
                debug!(
                    "building the last two layer: {}/{}",
                    source_id,
                    sub_graph.rows()
                );
            }
            let mut current_set = HashSet::new();
            let mut current_result = ResultType::default();
            for neighbor in row_vec.indices() {
                current_set.insert(NodeId {
                    layer: metapath[1],
                    id: *neighbor as u32,
                });
                current_result += node_features[&metapath[1]][*neighbor as usize].clone();
            }
            current_set.insert(NodeId {
                layer: metapath[0],
                id: source_id as u32,
            });
            current_result += node_features[&metapath[0]]
                .get(source_id)
                .cloned()
                .unwrap_or_default();
            context.push(current_set);
            results.push(current_result);
        }
        (context, results)
    } else {
        // the first layer to the last two layer

        let sub_graph = hetero_graph
            .subgraphs
            .get(&(metapath[0].to_string(), metapath[1].to_string()))
            .unwrap();
        let (previous_contexts, previous_results) =
            build_context_and_aggregation(hetero_graph, &metapath[1..], node_features);
        let mut contexts = Vec::with_capacity(sub_graph.rows());
        let mut results = Vec::with_capacity(sub_graph.rows());

        for (source_id, row_vec) in sub_graph.outer_iterator().enumerate() {
            if source_id % 1000 == 0 {
                debug!(
                    "building the first layer to the {} th layer: {}/{}",
                    metapath.len(),
                    source_id,
                    sub_graph.rows()
                );
            }
            let mut new_context = HashSet::new();
            let mut new_result = ResultType::default();
            for neighbor in row_vec.indices() {
                let pre_context = previous_contexts.get(*neighbor as usize);
                let pre_result = previous_results.get(*neighbor as usize);
                match pre_context {
                    Some(pre_context) => {
                        let current_size = new_context.len();
                        if current_size == 0 {
                            new_context = pre_context.clone();
                            new_result = pre_result.unwrap().clone();
                        } else {
                            new_result += pre_result.unwrap().clone();
                            for target_node in pre_context.iter() {
                                let inserted = new_context.insert(*target_node);
                                if !inserted {
                                    // already in the set!
                                    new_result -= node_features[&target_node.layer]
                                        [target_node.id as usize]
                                        .clone();
                                }
                            }
                        }
                    }
                    None => {
                        let inserted = new_context.insert(NodeId {
                            layer: metapath[1],
                            id: *neighbor as u32,
                        });
                        if inserted {
                            new_result += node_features[&metapath[1]]
                                .get(*neighbor as usize)
                                .cloned()
                                .unwrap_or_default();
                        }
                    }
                }
            }

            let inserted = new_context.insert(NodeId {
                layer: metapath[0],
                id: source_id as u32,
            });
            if inserted {
                new_result += node_features[&metapath[0]][source_id as usize].clone();
            }
            contexts.push(new_context);
            results.push(new_result);
        }
        (contexts, results)
    }
}
