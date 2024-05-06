use crate::error::Result;
use reqwest::header::LOCATION;
use url::Url;

use super::LinkCleaner;

pub struct YouShop10;

impl LinkCleaner for YouShop10 {
    async fn clean(&self, url: Url) -> Result<String> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let res = client.post(url).send().await?;
        Ok(res
            .headers()
            .get(LOCATION)
            .ok_or("Missing location header")?
            .to_str()?
            .to_string())
    }
}
