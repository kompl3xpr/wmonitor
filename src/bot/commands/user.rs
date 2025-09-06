use super::{Context, Error};

/// 用户操作
#[poise::command(
    prefix_command,
    slash_command,
    category = "用户",
    subcommands("join", "leave", "info")
)]
pub(super) async fn wmuser(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// 将用户添加进领地
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn join(
    ctx: Context<'_>,
    #[rename = "用户名"] username: String,
    #[rename = "领地名"] fief_name: String,
) -> Result<(), Error> {
    ctx.say(username).await?;
    Ok(())
}

/// 将用户从领地移出
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn leave(
    _ctx: Context<'_>,
    #[rename = "用户名"] username: String,
    #[rename = "领地名"] fief_name: String,
) -> Result<(), Error> {
    Ok(())
}

/// 获取用户信息
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn info(
    _ctx: Context<'_>,
    #[rename = "用户名"] username: String,

    #[rename = "领地名"]
    #[description = "不填则显示用户在所有领地的信息"]
    fief_name: Option<String>,
) -> Result<(), Error> {
    Ok(())
}
