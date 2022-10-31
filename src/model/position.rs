use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub title: String,
    pub company: String,
    pub start_date: String,
    pub end_date: Option<String>
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start_date.partial_cmp(&other.start_date)
    }
}