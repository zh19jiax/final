use std::collections::{VecDeque, HashMap};
use crate::data_loader::Freelancer;

/// Find connected components using BFS
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

/// Build adjacency list based on shared attributes
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

/// Count shared attributes between two freelancers
fn shared_attributes(a: &Freelancer, b: &Freelancer) -> f32 {
    let mut count = 0.0;
    if a.job_category == b.job_category { count += 0.3; }
    if a.platform == b.platform { count += 0.25; }
    if a.client_region == b.client_region { count += 0.25; }
    if a.experience_level == b.experience_level { count += 0.2; }
    count
}




#[test]
fn test_shared_attributes() {
    let f1 = Freelancer {
        id: 1, job_category: "Web Development".to_string(),
        platform: "Upwork".to_string(),
        client_region: "USA".to_string(),
        experience_level: "Expert".to_string(),
    };
    let f2 = Freelancer {
        id: 2, job_category: "Web Development".to_string(),
        platform: "Upwork".to_string(),
        client_region: "Europe".to_string(),
        experience_level: "Intermediate".to_string(),
    };
    assert_eq!(shared_attributes(&f1, &f2), 0.45);
}
