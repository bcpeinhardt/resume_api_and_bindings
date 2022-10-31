use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ContactInfo {
    pub name: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub github_handle: &'static str
}