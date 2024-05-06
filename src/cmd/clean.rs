use crate::{error::Result, links::try_parse, Context};
use url::Url;

#[poise::command(slash_command)]
pub async fn clean(ctx: Context<'_>, #[description = "URL to convert"] url: Url) -> Result<()> {
    ctx.defer().await?;

    let cleansed_url = try_parse(url).await?;

    ctx.say(cleansed_url).await?;

    Ok(())
}
