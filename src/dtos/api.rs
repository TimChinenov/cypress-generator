use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Api {
    pub summary: String,
    pub description: String
}

impl Api {
    pub fn print_path_summaries(&self) {
        println!("{}", self.summary)
    }
}