use reqwest::{header::HeaderMap, Client, Error, Response};
use serde::de::DeserializeOwned;

pub async fn fetch<T: DeserializeOwned>(url: &str, headers: HeaderMap) -> Result<T, Error> {
    let client = Client::new();
    let resp: Response = client.get(url).headers(headers).send().await?;
    return resp.json::<T>().await;
}
