/// Module for analyzing freelancer data and generating insights.
/// Provides functions for statistical analysis and data visualization.

use std::collections::HashMap;
use plotters::prelude::*;
use super::data_loader::Freelancer;

/// Analyzes performance metrics for each cluster of freelancers.
/// 
/// # Arguments: 
// `clusters` - Vector of clusters, where each cluster is a vector of freelancer indices, 
// `freelancers` - Slice of Freelancer structs to analyze
/// 
/// # Output
/// Prints analysis results including:
/// Number of members in each cluster
/// Average earnings per cluster
/// Average hourly rate per cluster
pub fn analyze_cluster_performance(clusters: &[Vec<usize>], freelancers: &[Freelancer]) {
    for (cluster_id, member_indices) in clusters.iter().enumerate() {
        let mut total_earnings = 0.0;
        let mut total_hourly = 0.0;
        let mut count = 0;

        // Calculate totals
        for &index in member_indices {
            if let Some(freelancer) = freelancers.get(index) {
                total_earnings += freelancer.earnings_usd;
                total_hourly += freelancer.hourly_rate;
                count += 1;
            }
        }

        // Calculate averages
        let avg_earnings = if count > 0 {
            total_earnings / count as f32
        } else {
            0.0
        };

        let avg_hourly = if count > 0 {
            total_hourly / count as f32
        } else {
            0.0
        };

        // Print results
        println!("Cluster {} Analysis:", cluster_id + 1);
        println!("- Members: {}", count);
        println!("- Average Earnings: ${:.2}", avg_earnings);
        println!("- Average Hourly Rate: ${:.2}\n", avg_hourly);
    }
}

/// Analyzes the profile characteristics of each cluster.
/// 
/// # Arguments
/// `clusters` - Vector of clusters, where each cluster is a vector of freelancer indices
/// `freelancers` - Slice of Freelancer structs to analyze
/// 
/// # Output
/// Prints dominant attributes for each cluster including:
///Job Category distribution
///Platform distribution
///Client Region distribution
///Experience Level distribution
pub fn analyze_cluster_profiles(clusters: &[Vec<usize>], freelancers: &[Freelancer]) {
    for (cluster_id, member_indices) in clusters.iter().enumerate() {
        let mut attributes = HashMap::new();
        let mut total_members = member_indices.len();
        
        for &idx in member_indices {
            let f = &freelancers[idx];
            *attributes.entry(("Job Category", f.job_category.clone())).or_insert(0) += 1;
            *attributes.entry(("Platform", f.platform.clone())).or_insert(0) += 1;
            *attributes.entry(("Region", f.client_region.clone())).or_insert(0) += 1;
            *attributes.entry(("Experience", f.experience_level.clone())).or_insert(0) += 1;
        }

        println!("\nCluster {} Profile ({} members):", cluster_id + 1, total_members);
        print_dominant_attributes(&attributes, "Job Category", total_members);
        print_dominant_attributes(&attributes, "Platform", total_members);
        print_dominant_attributes(&attributes, "Region", total_members);
        print_dominant_attributes(&attributes, "Experience", total_members);
    }
}

/// Prints the dominant attributes for a given category in a cluster.
/// 
/// # Arguments
/// `attributes` - Map of attribute categories and their frequencies
/// `category` - Category to analyze
/// `total` - Total number of members in the cluster
/// 
/// # Output
/// Prints the most common attribute and its percentage in the cluster
fn print_dominant_attributes(attributes: &HashMap<(&str, String), usize>, category: &str, total: usize) {
    let filtered: Vec<_> = attributes.iter()
        .filter(|((cat, _), _)| *cat == category)
        .collect();

    if let Some(((_ , val), count)) = filtered.iter().max_by_key(|(_, &count)| count) {
        let percentage = (**count as f32 / total as f32) * 100.0;
        println!("- Dominant {}: {} ({:.1}%)", category, val, percentage);
    }
}

/// Plots the hourly rates by experience level for each cluster.
/// 
/// # Arguments
/// `clusters` - Vector of clusters, where each cluster is a vector of freelancer indices
/// `freelancers` - Slice of Freelancer structs to analyze
/// 
/// # Returns
/// `Result<(), Box<dyn Error>>` - Success or error status
/// 
/// # Output
/// Generates a bar chart saved as "cluster_experience_rates.png" showing:
/// Hourly rates grouped by experience level
/// Different colors for each experience level
/// Cluster IDs on x-axis
/// Average hourly rates on y-axis
pub fn plot_cluster_experience_rates(
    clusters: &[Vec<usize>],
    freelancers: &[Freelancer],
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Prepare data structure
    let mut cluster_data = Vec::new();
    let experience_levels = ["Beginner", "Intermediate", "Expert"];
    let colors = [RGBColor(255, 0, 0), RGBColor(0, 255, 0), RGBColor(0, 0, 255)];

    for (cluster_id, members) in clusters.iter().enumerate() {
        let mut exp_rates = HashMap::new();
        let mut counts = HashMap::new();

        // Calculate averages per experience level
        for &member_idx in members {
            let f = &freelancers[member_idx];
            *exp_rates.entry(f.experience_level.as_str()).or_insert(0.0) += f.hourly_rate;
            *counts.entry(f.experience_level.as_str()).or_insert(0) += 1;
        }

        let mut cluster_rates = Vec::new();
        for exp in &experience_levels {
            let avg = counts.get(*exp)
                .and_then(|&c| if c > 0 { Some(exp_rates[*exp] / c as f32) } else { None })
                .unwrap_or(0.0);
            cluster_rates.push(avg);
        }

        cluster_data.push((cluster_id, cluster_rates));
    }

    // 2. Create the chart with continuous x-axis
    let root = BitMapBackend::new("cluster_experience_rates.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_rate = cluster_data.iter()
        .flat_map(|(_, rates)| rates.iter())
        .fold(f32::NAN, |a, &b| a.max(b)) * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Hourly Rates by Experience Level per Cluster", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0.0..clusters.len() as f64,  // Continuous x-axis
            0.0..max_rate as f64         // Continuous y-axis
        )?;

    chart.configure_mesh()
        .x_desc("Cluster ID")
        .y_desc("Average Hourly Rate (USD)")
        .bold_line_style(&BLACK.mix(0.2))
        .x_labels(15)
        .draw()?;

    // 3. Draw grouped bars with proper coordinate types
    let bar_width = 0.2;
    for (exp_idx, exp) in experience_levels.iter().enumerate() {
        let x_offset = (exp_idx as f64 - 1.0) * bar_width;

        chart.draw_series(
            cluster_data.iter().map(|(cluster_id, rates)| {
                let x_center = *cluster_id as f64 + x_offset;
                let y_value = rates[exp_idx] as f64;
                
                Rectangle::new(
                    [
                        (x_center - bar_width/2.0, 0.0),  // Left edge
                        (x_center + bar_width/2.0, y_value) // Right edge
                    ],
                    colors[exp_idx].filled(),
                )
            })
        )?.label(*exp);
    }

    // 4. Add legend and finalize
    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}