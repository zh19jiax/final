mod data_loader;
mod algorithms;

use data_loader::load_freelancers;
use algorithms::{build_collaboration_graph, find_connected_components};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data
    let freelancers = load_freelancers("data/freelancers.csv")?;
    
    // Build collaboration graph
    let adj_list = build_collaboration_graph(&freelancers);
    
    // Find connected components using BFS
    let clusters = find_connected_components(&adj_list);
    
    // Print results
    println!("Found {} clusters", clusters.len());
    for (i, cluster) in clusters.iter().enumerate() {
        let freelancer_ids: Vec<u32> = cluster.iter()
            .map(|&idx| freelancers[idx].id)
            .collect();
        println!("Cluster {}: {:?}", i + 1, freelancer_ids);
    }
    
    Ok(())
}