use std::path::Path;
use std::io;
use serde::{Deserialize, Serialize};
use csv::Reader;
use toml;
use std::fs;

pub fn is_authorized_key(key: &String) -> bool {
    // Just check the config.toml exists for the endpoint
    if key.contains("..") || key.contains("/") {
        return false;
    }else{
     let path = Path::new("config").join(&key).join("config.toml");
     path.exists() 
    }
}
pub fn is_authorized_key_and_secret(key: &String, secret: &String) -> bool {
    // Check if the key is correct and the secret matches
    if is_authorized_key(&key) {
        let kc = key_configuration(&key).expect("Failed to load config");
         kc.password == secret.to_string()
    }else{
        false
    }
}


pub fn key_configuration( key: &String) -> Result<KeyConfig, String> {
    // Pull in the general configuration
    let path = Path::new("config").join(&key).join("config.toml");
    if path.exists()  {
        let cont =fs::read_to_string(path).expect("Failed to read config");
        let configuration: KeyConfig = toml::from_str(&cont).expect("Failed to parse config");

        Ok(configuration)
    }else{
        Err("Missing Config".to_string())
    }
}

pub fn get_status(key: &String) -> Result<StatusInfo, String> {
    //Load the status from file
    let status_path = Path::new("config").join(&key).join("status.txt");
    let media_path = Path::new("config").join(&key).join("media.txt");
    let status = fs::read_to_string(status_path).expect("Failed to read status");
    let media = fs::read_to_string(media_path).expect("Failed to read media_url");

    return Ok(StatusInfo { text: status, media_url: media});
}


pub fn save_status (key: &String, si: &StatusInfo) -> Result<bool, String> {
    //Save our status to file
    let status_path = Path::new("config").join(&key).join("status.txt");
    let media_path = Path::new("config").join(&key).join("media.txt");

    fs::write(status_path,&si.text).expect("Failed to save status");
    fs::write(media_path,&si.media_url).expect("Failed to save media_url");
    Ok(true)
}

pub fn location_info( longitude: f64, latitude: f64, key: &String) -> Result<StatusInfo, String> {
    // Parse the locations and return the specific text
    let path = Path::new("config").join(&key).join("locations.csv");
    if path.exists() {
        let mut loc_rdr = csv::Reader::from_path(path).unwrap();
        //Check all the lines for a matching location
        for result in loc_rdr.deserialize() {
            let si: TrackServerInfo = result.unwrap();
            
            if  check_location(longitude, latitude, si.long_1, si.long_2, si.lat_1, si.lat_2) {
                    //We are inside the bounding box
                    let rsi = StatusInfo { text: si.text, media_url: si.media_url};
                    save_status(&key,&rsi)?;
                    return Ok(rsi);
            }
        }
        //Default config
        let conf = key_configuration(&key)?;
        let dsi = StatusInfo { text: conf.default_status, media_url: conf.default_media_url};
        save_status(&key,&dsi)?;
        return Ok(dsi);
        
    }else{ 
        return Err("Config not found".to_string());
    }
}

fn check_location (longitude: f64, latitude: f64, long_1:f64, long_2:f64, lat_1:f64, lat_2:f64) -> bool {
    return ((long_1 < longitude && long_2 > longitude) || (long_1 > longitude && long_2 < longitude)) &&
            (lat_1 < latitude && lat_2 > latitude) || (lat_1 > latitude && lat_2 < latitude);
}

#[derive(Serialize, Deserialize)]
pub struct KeyConfig {
    pub name: String,
    pub title: String,
    pub font: String,
    pub generate_kindle: bool,
    password: String,
    default_status: String,
    default_media_url: String,
}

#[derive(Deserialize)]
pub struct TrackServerInfo {
    long_1: f64,
    long_2: f64,
    lat_1: f64,
    lat_2: f64,
    text: String,
    media_url: String,
}

#[derive(Deserialize)]
pub struct StatusInfo {
    pub text: String,
    pub media_url: String,
}