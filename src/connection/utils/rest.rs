use super::enums::RestMethod;
use reqwest;
use serde::de::DeserializeOwned;

pub async fn fetch<T: DeserializeOwned>(url: String, method: RestMethod) -> Result<T, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = match method {
        RestMethod::GET => {
            client.get(&url)
        },
        RestMethod::POST => {
            client.post(&url)
        },
        RestMethod::PUT => {
            client.put(&url)
        },
        RestMethod::DELETE => {
            client.delete(&url)
        }
    };

    let resp = resp
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(resp)
}