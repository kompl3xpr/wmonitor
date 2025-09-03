use anyhow::Result;


#[derive(typed_builder::TypedBuilder)]
pub struct WMonitor {
    repo: crate::repos::Repositories,
    bot: serenity::Client,
}

impl WMonitor {
    pub async fn run(mut self) -> Result<()> {
        self.bot.start().await?;
        Ok(())
    }
}