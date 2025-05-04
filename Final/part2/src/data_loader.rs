/// Module for loading and processing freelancer data from CSV files.

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

/// Represents a freelancer with their professional attributes and performance metrics.
/// 
/// # Fields
/// `id` - Unique identifier for the freelancer
/// `job_category` - Type of work the freelancer specializes in
/// `platform` - Freelancing platform where the freelancer operates
/// `experience_level` - Level of professional experience
/// `client_region` - Geographic region of the freelancer's clients
/// `earnings_usd` - Total earnings in USD
/// `hourly_rate` - Charged hourly rate in USD
/// `job_success_rate` - Percentage of successfully completed jobs
pub struct Freelancer {
    pub id: u32,
    pub job_category: String,
    pub platform: String,
    pub experience_level: String,
    pub client_region: String,
    pub earnings_usd: f32,
    pub hourly_rate: f32,
    pub job_success_rate: f32,
}

/// Loads freelancer data from a CSV file.
/// 
/// # Arguments: `path` - Path to the CSV file containing freelancer data
/// 
/// # Returns: `Result<Vec<Freelancer>, Box<dyn Error>>` - Vector of parsed freelancer data or error
/// 
/// # Errors
/// Returns error if file cannot be opened or read, CSV parsing fails, or data conversion fails
pub fn load_freelancers(path: &str) -> Result<Vec<Freelancer>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    
    let mut freelancers = Vec::new();
    for result in rdr.records() {
        let record = result?;
        
        let freelancer = Freelancer {
            id: record[0].parse()?,
            job_category: record[1].to_string(),
            platform: record[2].to_string(),
            experience_level: record[3].to_string(),
            client_region: record[4].to_string(),
            earnings_usd: record[7].parse()?,
            hourly_rate: record[8].parse()?,
            job_success_rate: record[9].parse()?,
        };
        
        freelancers.push(freelancer);
    }
    
    Ok(freelancers)
}
