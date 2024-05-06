use crate::error::Result;
use url::Url;

use self::youshop::YouShop10;

pub mod youshop;

pub async fn try_parse(url: Url) -> Result<String> {
    let host_str = url.host_str().ok_or("Missing host in URL")?;

    match host_str {
        "k.youshop10.com" => Ok(YouShop10.clean(url).await?),
        _ => Err("Invalid host".into()),
    }
}

pub trait LinkCleaner {
    fn clean(&self, url: Url) -> impl std::future::Future<Output = Result<String>>;
}
