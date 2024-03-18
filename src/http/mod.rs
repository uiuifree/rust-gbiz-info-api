use std::fmt::{Debug};
use crate::error::GBizApiError;


#[derive(Default, Debug)]
pub struct HttpClient {}


impl HttpClient {
    pub async fn get<T, U>(token: &str, url: &str, params: U) -> Result<T, GBizApiError>
        where
            T: for<'de> serde::Deserialize<'de>,
            U: serde::Serialize + std::fmt::Debug
    {
        let response = reqwest::Client::new()
            .get(url)
            .header("X-hojinInfo-api-token", token)
            .header("Accept", "application/json")
            .query(&params)
            .send()
            .await;

        let response = match response {
            Ok(v) => { v }
            Err(err) => { return Err(GBizApiError::Connection(err.to_string())); }
        };
        let status = response.status();
        let value = match response.text().await {
            Ok(v) => { v }
            Err(e) => {
                return Err(GBizApiError::JsonParse(e.to_string()));
            }
        };
        if status != 200 {
            return Err(GBizApiError::JsonParse(value));
        }
        let parse = serde_json::from_str(value.as_str());
        if parse.is_err() {
            return Err(GBizApiError::JsonParse(value));
        }

        Ok(parse.unwrap())
    }
}