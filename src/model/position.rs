use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    title: &'static str,
    company: &'static str,
    start_date: &'static str,
    end_date: Option<&'static str>
}

impl Position {
    pub fn is_current(&self) -> bool {
        self.end_date.is_none()
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start_date.partial_cmp(other.start_date)
    }
}