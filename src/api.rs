use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use reqwest::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.apilayer.com/exchangerates_data";
const API_KEY: &str = "CeT6KUeYIVle0FlH47u5MtnLKS9RdfhF";

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct Motd {
    pub msg: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct ConvertResponse {
    pub success: Option<bool>,
    pub date: Option<String>,
    pub result: Option<f64>,
    pub query: Option<Query>,
    pub info: Option<Info>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct Query {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct Info {
    pub rate: f64,
}

pub struct RatesApi {
    client: Client,
}

impl RatesApi {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("currency-cli/0.1")
            .build()?;
        Ok(Self { client })
    }

    /// Convert a given amount from one currency to another using the API's `/convert` endpoint
    pub async fn convert(&self, amount: f64, from: &str, to: &str) -> Result<ConvertResponse> {
        let url = format!(
            "{BASE_URL}/convert?from={}&to={}&amount={}",
            from.to_uppercase(),
            to.to_uppercase(),
            amount
        );
        let resp = self.client
            .get(&url)
            .header("apikey", API_KEY)
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(anyhow!("API error: {}", resp.status()));
        }
        let parsed: ConvertResponse = resp.json().await?;
        println!(
            "Conversion {} {} -> {}: {:?}",
            amount,
            from.to_uppercase(),
            to.to_uppercase(),
            parsed.result
        );
        Ok(parsed)
    }

    /// Historical conversion on a specific date
    pub async fn historical_convert(
        &self,
        amount: f64,
        from: &str,
        to: &str,
        date: NaiveDate,
    ) -> Result<ConvertResponse> {
        let url = format!(
            "{BASE_URL}/convert?from={}&to={}&amount={}&date={}",
            from.to_uppercase(),
            to.to_uppercase(),
            amount,
            date
        );
        let resp = self.client
            .get(&url)
            .header("apikey", API_KEY)
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(anyhow!("API error: {}", resp.status()));
        }
        let parsed: ConvertResponse = resp.json().await?;
        println!(
            "Historical conversion {} {} -> {} on {}: {:?}",
            amount,
            from.to_uppercase(),
            to.to_uppercase(),
            date,
            parsed.result
        );
        Ok(parsed)
    }
}