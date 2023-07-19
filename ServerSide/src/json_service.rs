use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::models::Profile;

pub fn get_profile_from_json() -> Result<Profile, Box<dyn std::error::Error>>{
    let json_file_path = "MockedProfile.json";
    let contents = std::fs::read_to_string(json_file_path)?;
    let profile: Profile = serde_json::from_str(&contents)?;

    Ok(profile)
}