use std::collections::HashMap;
use reqwest::Error;
use serde::de::DeserializeOwned;

/**
 * Gets an item from deconz based on params. (eg.: /sensors, /sensor/{id})
 */
pub async fn get<T> (param: String) -> Result<T, Error> where T: DeserializeOwned {
    let request_url = format!(
        "{base_url}/api/{api_key}{param}", 
        base_url = crate::CONFIG.api_url,
        api_key = crate::CONFIG.api_key);

    //println!("GET: {}", request_url);
    let temp_sensor_response = reqwest::get(request_url)
        .await?
        .json::<T>()
        .await?;
    
    Ok(temp_sensor_response)
}

/**
 * Sends an action to deconz. (eg.: switch on light)
 */
pub async fn put(action: String, body: HashMap<String, bool>) -> Result<reqwest::Response, Error> {
    let request_url = format!(
        "{base_url}/api/{api_key}{action}", 
        base_url = crate::CONFIG.api_url,
        api_key = crate::CONFIG.api_key);

    let client = reqwest::Client::new();
    let resp = client
        .put(request_url)
        .json(&body)
        .send()
        .await
        .unwrap();    
    
    Ok(resp)
}