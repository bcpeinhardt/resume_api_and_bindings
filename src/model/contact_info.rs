use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactInfo {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub github_handle: String
}