/// Module implementing various algorithms for freelancer data analysis.

use std::collections::VecDeque;
use super::data_loader::Freelancer;

/// Finds connected components in a graph using Breadth-First Search (BFS).
/// 
/// # Arguments: `adj_list` - Adjacency list representation of the graph
/// 
/// # Returns: `Vec<Vec<usize>>` - Vector of clusters, where each cluster is a vector of node indices
pub fn find_connected_components(adj_list: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut visited = vec![false; adj_list.len()];
    let mut clusters = Vec::new();

    for node in 0..adj_list.len() {
        if !visited[node] {
            let mut cluster = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(node);
            visited[node] = true;

            while let Some(current) = queue.pop_front() {
                cluster.push(current);
                for &neighbor in &adj_list[current] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
            clusters.push(cluster);
        }
    }
    clusters
}

/// Builds a collaboration graph based on shared attributes between freelancers.
/// 
/// # Arguments: `freelancers` - Slice of Freelancer structs to analyze
/// 
/// # Returns: `Vec<Vec<usize>>` - Adjacency list representation of the collaboration graph
pub fn build_collaboration_graph(freelancers: &[Freelancer]) -> Vec<Vec<usize>> {
    let n = freelancers.len();
    let mut adj_list = vec![Vec::new(); n];

    for i in 0..n {
        for j in (i + 1)..n {
            if shared_attributes(&freelancers[i], &freelancers[j]) > 0.7 {
                adj_list[i].push(j);
                adj_list[j].push(i);
            }
        }
    }
    adj_list
}

/// Calculates similarity score between two freelancers based on shared attributes.
/// 
/// # Arguments: `a` - First freelancer to compare, `b` - Second freelancer to compare
/// 
/// # Returns: `f32` - Similarity score between 0.0 and 1.
fn shared_attributes(a: &Freelancer, b: &Freelancer) -> f32 {
    let mut count = 0.0;
    if a.job_category == b.job_category { count += 0.3; }
    if a.platform == b.platform { count += 0.25; }
    if a.client_region == b.client_region { count += 0.25; }
    if a.experience_level == b.experience_level { count += 0.2; }
    count
}


/// Creates test data for unit testing
fn create_test_freelancers() -> Vec<Freelancer> {
    vec![
        Freelancer {
            id: 1,
            job_category: "Web Development".to_string(),
            platform: "Upwork".to_string(),
            client_region: "USA".to_string(),
            experience_level: "Expert".to_string(),
            earnings_usd: 0.0,
            hourly_rate: 0.0,
        },
        Freelancer {
            id: 2,
            job_category: "Web Development".to_string(),
            platform: "Upwork".to_string(),
            client_region: "USA".to_string(),
            experience_level: "Expert".to_string(),
            earnings_usd: 0.0,
            hourly_rate: 0.0,
        },
        Freelancer {
            id: 3,
            job_category: "Design".to_string(),
            platform: "Fiverr".to_string(),
            client_region: "Europe".to_string(),
            experience_level: "Beginner".to_string(),
            earnings_usd: 0.0,
            hourly_rate: 0.0,
        },
    ]
}

/// Tests finding connected components in a simple graph
#[test]
fn test_find_connected_components() {
    let adj_list = vec![
        vec![1],     
        vec![0, 2],  
        vec![1],   
        vec![],    
    ];
    
    let clusters = find_connected_components(&adj_list);
    assert_eq!(clusters.len(), 2);  // Should have 2 clusters
    assert_eq!(clusters[0].len(), 3);  // First cluster has 3 nodes
    assert_eq!(clusters[1].len(), 1);  // Second cluster has 1 node
}

/// Tests building collaboration graph with similar freelancers
#[test]
fn test_build_collaboration_graph() {
    let freelancers = create_test_freelancers();
    let graph = build_collaboration_graph(&freelancers);
    
    // First two freelancers should be connected (identical attributes)
    assert!(graph[0].contains(&1));
    assert!(graph[1].contains(&0));
    
    // Third freelancer should not be connected to others (different attributes)
    assert!(!graph[0].contains(&2));
    assert!(!graph[1].contains(&2));
}

/// Tests shared attributes calculation
#[test]
fn test_shared_attributes() {
    let f1 = Freelancer {
        id: 1,
        job_category: "Web Development".to_string(),
        platform: "Upwork".to_string(),
        client_region: "USA".to_string(),
        experience_level: "Expert".to_string(),
        earnings_usd: 0.0,
        hourly_rate: 0.0,
    };
    
    let f2 = Freelancer {
        id: 2,
        job_category: "Web Development".to_string(),
        platform: "Upwork".to_string(),
        client_region: "Europe".to_string(),
        experience_level: "Intermediate".to_string(),
        earnings_usd: 0.0,
        hourly_rate: 0.0,
    };
    
    // Should have 0.55 similarity (0.3 + 0.25)
    assert_eq!(shared_attributes(&f1, &f2), 0.55);
}

