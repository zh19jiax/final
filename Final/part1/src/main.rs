mod data_loader;
mod algorithms;
mod analysis;

use data_loader::load_freelancers;
use algorithms::{build_collaboration_graph, find_connected_components};
use analysis::{analyze_cluster_performance, analyze_cluster_profiles, plot_cluster_experience_rates};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data
    let freelancers = load_freelancers("freelancer_data.csv")?;
    
    // Build collaboration graph
    let adj_list = build_collaboration_graph(&freelancers);
    
    // Find connected components using BFS
    let clusters = find_connected_components(&adj_list);
    
    // Print analysis
    analyze_cluster_performance(&clusters, &freelancers);

    analyze_cluster_profiles(&clusters, &freelancers);

    plot_cluster_experience_rates(&clusters, &freelancers);

    Ok(())
}