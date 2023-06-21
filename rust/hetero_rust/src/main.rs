use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let tasks = hetero_rust::get_all_taskpack();
    let mut all_results = vec![];
    for task in tasks {
        println!("task: {:?}", task.graph_name);
        let graph = hetero_rust::load_hetero_graph(&task.graph_path)
            .unwrap_or_else(|e| panic!("load graph {:?} error: {:?}", task.graph_path, e));
        let result = task
            .metapathes
            .metapathes
            .par_iter()
            .map(|meta_path| {
                // println!("meta_path: {:?}", meta_path);
                let memory_usage = hetero_rust::count_metapath_memory_usage(&graph, &meta_path);
                // println!("memory_usage: {:?}", memory_usage);
                (&task.graph_name, meta_path, memory_usage)
            })
            .collect::<Vec<_>>();
        for (graph_name, meta_path, memory_usage) in &result {
            println!(
                "graph_name: {:?}, meta_path: {:?}, memory_usage: {:?}",
                graph_name, meta_path, memory_usage
            );
        }
        all_results.extend(
            result
                .iter()
                .map(|(name, path, usage)| ((**name).clone(), (**path).clone(), *usage)),
        );
    }
    for (graph_name, meta_path, memory_usage) in &all_results {
        println!(
            "graph_name: {:?}, meta_path: {:?}, memory_usage: {:?}",
            graph_name, meta_path, memory_usage
        );
    }
}
