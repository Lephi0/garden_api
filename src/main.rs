use std::collections::HashMap;

use once_cell::sync::Lazy;
use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use toml;

mod mappings;
use crate::mappings::sensor::Sensor;
use crate::mappings::group::Group;

struct AppSensor {
    id: String,
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    api_url: String,
    api_key: String,
}

/**
 * Load global config from config.toml file.
 */
static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    let f = std::fs::read_to_string("config.toml").unwrap();
    let config: AppConfig = toml::from_str(&f).unwrap();
    config
});

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let f = std::fs::read_to_string("config.toml").unwrap();
    // let config: AppConfig = toml::from_str(&f).unwrap();

    println!("{:?}", CONFIG.api_url);

    let sensor_list_response = get::<HashMap<String, Sensor>>(format!("/sensors")).await?;

    //println!("{:#?}", sensor_list_response);

    let mut s_temp_id: Option<String> = None;
    let mut s_lux_id: Option<String> = None;
    let mut s_hum_id: Option<String> = None;
    let mut s_pres_id: Option<String> = None;

    // Get sensor ID-s
    for (id, sensor) in &sensor_list_response {
        if sensor.r#type.is_some() {
            if sensor.r#type.as_ref().unwrap().contains("Temperature") {
                s_temp_id = Some(id.to_string());
            }
            if sensor.r#type.as_ref().unwrap().contains("LightLevel") {
                s_lux_id = Some(id.to_string());
            }
            if sensor.r#type.as_ref().unwrap().contains("Humidity") {
                s_hum_id = Some(id.to_string());
            }
            if sensor.r#type.as_ref().unwrap().contains("Pressure") {
                s_pres_id = Some(id.to_string());
            }
        }
    }

    // Temperature
    if s_temp_id.is_some() {
        let s_temp_id = s_temp_id.unwrap();
        //println!("Temperature sensor ID: {}", s_temp_id);
        let temp_sensor = get::<Sensor>(format!("/sensors/{s_temp_id}"));
        let resp = temp_sensor.await?;

        println!("Temperature: {:#?} °C", trim_temp(resp.state.temperature.unwrap()));
    }

    // Light itensity
    if s_lux_id.is_some() {
        let s_lux_id = s_lux_id.unwrap();
        //println!("Light sensor ID: {}", s_lux_id);
        let lux_sensor = get::<Sensor>(format!("/sensors/{s_lux_id}"));
        let resp = lux_sensor.await?;

        println!("Light Level: {:#?} lux", resp.state.lux.unwrap());
    }
    
    // Humidity
    if s_hum_id.is_some() {
        let s_hum_id = s_hum_id.unwrap();
        //println!("Temperature sensor ID: {}", s_temp_id);
        let hum_sensor = get::<Sensor>(format!("/sensors/{s_hum_id}"));
        let resp = hum_sensor.await?;

        println!("Humidity: {:#?} %", trim_temp(resp.state.humidity.unwrap()));
    }

    // Pressure
    if s_pres_id.is_some() {
        let s_pres_id = s_pres_id.unwrap();
        //println!("Temperature sensor ID: {}", s_temp_id);
        let pres_sensor = get::<Sensor>(format!("/sensors/{s_pres_id}"));
        let resp = pres_sensor.await?;

        println!("Pressure: {:#?} hPa", resp.state.pressure.unwrap());
    }

    let groups_response = get::<HashMap<String, Group>>(format!("/groups")).await?;
    let mut group_id: Option<String> = None;
    // Get Garden group ID-s
    for (id, group) in &groups_response {
        if group.name.contains("Garden") {
            group_id = Some(id.to_string());
        }
    }

    // Toggle group action
    // if group_id.is_some() {
    //     let group_id = group_id.unwrap();
    //     println!("\n-----\nGroup ID: {}", group_id);
    //     let mut action: HashMap<String, bool> = HashMap::new();
    //     //action.insert("on", "false");
    //     action.insert("toggle".to_string(), true);

    //     let response = put(format!("/groups/{group_id}/action"), action).await?;
    //     println!("Success: {:#?}", response.status().is_success());
    // }

    Ok(())
}

async fn get<T> (param: String) -> Result<T, Error> where T: DeserializeOwned {
    let request_url = format!(
        "{base_url}/api/{api_key}{param}", 
        base_url = CONFIG.api_url,
        api_key = CONFIG.api_key);

    //println!("GET: {}", request_url);
    let temp_sensor_response = reqwest::get(request_url)
        .await?
        .json::<T>()
        .await?;
    
    Ok(temp_sensor_response)
}

async fn put(action: String, body: HashMap<String, bool>) -> Result<reqwest::Response, Error> {
    let request_url = format!(
        "{base_url}/api/{api_key}{action}", 
        base_url = CONFIG.api_url,
        api_key = CONFIG.api_key);

    let client = reqwest::Client::new();
    let resp = client
        .put(request_url)
        .json(&body)
        .send()
        .await
        .unwrap();    
    
    Ok(resp)
}

/**
 * Trims the first two digit from an integer. Api gives temperature value
 * as "2502" so we convert it to "25" only.
 */
fn trim_temp(value: u32) -> u32 {
    let trimmed_value: String = value.to_string().chars().into_iter().take(2).collect();
    trimmed_value.parse::<u32>().unwrap()
}
