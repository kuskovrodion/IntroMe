use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {
    pub personal_info: PersonalInfo,
    pub work_info: WorkInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersonalInfo {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
    pub email: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorkInfo {
    pub position: String
}