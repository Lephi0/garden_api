use std::collections::HashMap;
use std::time::Duration;

use chrono::prelude::*;
use tokio::{task, time};
use once_cell::sync::Lazy;
use reqwest::Error;
use serde::{Serialize, Deserialize};
use toml;

mod mappings;
use crate::mappings::sensor::Sensor;
use crate::mappings::group::Group;
mod deconz;
use crate::deconz::client;

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

static WAIT_FOR_SEC: u64 = 10;
static LUX_THRESHOLD: u32 = 1000;
static SEVEN_AM: u32 = 7;
static TEN_PM: u32 = 22;

#[tokio::main]
async fn main() {
    let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(WAIT_FOR_SEC));

        loop {
            let mut lux_value = 0;
            interval.tick().await;
            read_sensors(&mut lux_value).await;
            toggle_led(&lux_value).await;
        }
    });

    forever.await;
}

async fn read_sensors(lux_value: &mut u32) -> Result<(), Error> {
    // TODO calling sensors should be a parameter value
    // TODO .is_success() and error handling
    let sensor_list_response = client::get::<HashMap<String, Sensor>>(format!("/sensors")).await?;

    let (
        mut s_temp_id,
        mut s_hum_id,
        mut s_pres_id,
        mut s_lux_id,
    ) = (None, None, None, None);

    // Get sensor ID-s
    for (id, sensor) in &sensor_list_response {
        if sensor.r#type.is_some() {
            if sensor.r#type.as_ref().unwrap().contains("Temperature") {
                s_temp_id = Some(id.to_string());
            }
            if sensor.r#type.as_ref().unwrap().contains("Humidity") {
                s_hum_id = Some(id.to_string());
            }
            if sensor.r#type.as_ref().unwrap().contains("Pressure") {
                s_pres_id = Some(id.to_string());
            }
            if sensor.r#type.as_ref().unwrap().contains("LightLevel") {
                s_lux_id = Some(id.to_string());
            }
        }
    }

    // Temperature
    if s_temp_id.is_some() {
        let s_temp_id = s_temp_id.unwrap();
        //println!("Temperature sensor ID: {}", s_temp_id);
        let temp_sensor = client::get::<Sensor>(format!("/sensors/{s_temp_id}"));
        let resp = temp_sensor.await?;

        println!("Temperature: {:#?} °C", trim_temp(resp.state.temperature.unwrap()));
    }
    
    // Humidity
    if s_hum_id.is_some() {
        let s_hum_id = s_hum_id.unwrap();
        //println!("Temperature sensor ID: {}", s_temp_id);
        let hum_sensor = client::get::<Sensor>(format!("/sensors/{s_hum_id}"));
        let resp = hum_sensor.await?;

        println!("Humidity: {:#?} %", trim_temp(resp.state.humidity.unwrap()));
    }

    // Pressure
    if s_pres_id.is_some() {
        let s_pres_id = s_pres_id.unwrap();
        //println!("Temperature sensor ID: {}", s_temp_id);
        let pres_sensor = client::get::<Sensor>(format!("/sensors/{s_pres_id}"));
        let resp = pres_sensor.await?;

        println!("Pressure: {:#?} hPa", resp.state.pressure.unwrap());
    }

    // Light itensity
    if s_lux_id.is_some() {
        let s_lux_id = s_lux_id.unwrap();
        //println!("Light sensor ID: {}", s_lux_id);
        let lux_sensor = client::get::<Sensor>(format!("/sensors/{s_lux_id}"));
        let resp = lux_sensor.await?;

        *lux_value = resp.state.lux.unwrap();
        println!("Light Level: {:#?} lux", lux_value);
    }
    println!("----");

    Ok(())
}

async fn toggle_led(lux_value: &u32) -> Result<(), Error> {
    // Get group "Garden"
    let groups_response = client::get::<HashMap<String, Group>>(format!("/groups")).await?;
    let mut group_id: Option<String> = None;
    // Get Garden group ID-s
    for (id, group) in &groups_response {
        if group.name.contains("Garden") {
            group_id = Some(id.to_string());
        }
    }
    
    // Toggle group action
    if group_id.is_some() {
        let group_id = group_id.unwrap();
        let group = client::get::<Group>(format!("/groups/{group_id}")).await?;

        // Turn the light on only in a specific timeframe.
        if is_in_timeframe() {
            // Switch light only when it is not only in that state (if lux > 1000 and it is not already turned on)
            if *lux_value > LUX_THRESHOLD && group.state.all_on == true {
                switch_light_on(group_id, false).await?;
            } else if *lux_value < LUX_THRESHOLD && group.state.all_on == false {
                switch_light_on(group_id, true).await?;
            }
        } else if group.state.all_on == true {
            switch_light_on(group_id, false).await?;
        }
    }

    Ok(())
}

fn is_in_timeframe() -> bool {
    let now: DateTime<Utc> = Utc::now() + chrono::Duration::hours(2);

    let seven_am = Utc.ymd(now.year(), now.month(), now.day()).and_hms(13, 23, 0);
    let ten_pm = Utc.ymd(now.year(), now.month(), now.day()).and_hms(TEN_PM, 0, 0);

    if now > seven_am && now < ten_pm {
        return true;
    } else {
        return false;
    }
}

/**
 * Sets the group action on to true or false.
 */
async fn switch_light_on(group_id: String, on: bool) -> Result<(), Error> {
    println!(" - Switching on: {} - ", on);
    let mut action: HashMap<String, bool> = HashMap::new();
    action.insert("on".to_string(), on);

    let response = client::put(format!("/groups/{group_id}/action"), action).await?;
    // println!("Success: {:#?}", response.status().is_success());

    Ok(())
}

/**
 * Trims the first two digit from an integer. Api gives temperature value
 * as "2502" so we convert it to "25" only.
 */
fn trim_temp(value: u32) -> u32 {
    let trimmed_value: String = value.to_string().chars().into_iter().take(2).collect();
    trimmed_value.parse::<u32>().unwrap()
}
