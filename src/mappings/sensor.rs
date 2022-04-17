use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub on: bool,
    pub reachable: Option<bool>,
    pub battery: Option<u32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct State {
    pub lastupdated: Option<String>,
    pub temperature: Option<u32>,
    pub lux: Option<u32>,
    pub humidity: Option<u32>,
    pub pressure: Option<u32>,

}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Sensor {
    pub r#type: Option<String>,
    pub name: String,
    pub config: Config,
    pub state: State,
}