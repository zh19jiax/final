use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
pub struct Freelancer {
    pub id: u32,
    pub job_category: String,
    pub platform: String,
    pub client_region: String,
    pub experience_level: String,
    pub earnings_usd: f32,
    pub hourly_rate: f32,
}

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
            client_region: record[4].to_string(),
            experience_level: record[3].to_string(),
            earnings_usd: record[7].parse()?,
            hourly_rate: record[8].parse()?,
        };
        freelancers.push(freelancer);
    }
    Ok(freelancers)
}
