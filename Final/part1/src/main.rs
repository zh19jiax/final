/// Main module for the freelancer data analysis system.

use std::error::Error;
use data_loader::load_freelancers;
use algorithms::{build_collaboration_graph, find_connected_components};
use analysis::{analyze_cluster_performance, analyze_cluster_profiles, plot_cluster_experience_rates};

mod data_loader;
mod algorithms;
mod analysis;

/// Main function that demonstrates the data analysis workflow.
/// 1. Loads freelancer data from CSV file
/// 2. Builds collaboration graph based on shared attributes
/// 3. Finds connected components (clusters) in the graph
/// 4. Analyzes cluster performance and profiles
/// 5. Generates visualization of hourly rates by experience level

fn main() -> Result<(), Box<dyn Error>> {
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