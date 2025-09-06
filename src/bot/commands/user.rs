use crate::bot::commands::id_of;

use super::{Context, Error};

/// 用户操作
#[poise::command(
    prefix_command,
    slash_command,
    category = "用户",
    subcommands("join", "leave", "allow", "deny", "info")
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
    let repo = &ctx.data().repo;

    // try to create user in db
    let user_id = id_of(ctx.author());
    repo.user().create(user_id, false).await?;

    ctx.say(username).await?;
    Ok(())
}

/// 将用户从领地移出
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn leave(
    ctx: Context<'_>,
    #[rename = "用户名"] username: String,
    #[rename = "领地名"] fief_name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    // try to create user in db
    let user_id = id_of(ctx.author());
    repo.user().create(user_id, false).await?;

    Ok(())
}

/// 收回用户对领地的权限
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn deny(
    ctx: Context<'_>,
    #[rename = "用户名"] username: String,

    #[rename = "领地名"]
    #[description = "不填则显示用户在所有领地的信息"]
    fief_name: Option<String>,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    // try to create user in db
    let user_id = id_of(ctx.author());
    repo.user().create(user_id, false).await?;

    Ok(())
}

/// 给予用户对领地的权限
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn allow(
    ctx: Context<'_>,
    #[rename = "用户名"] username: String,

    #[rename = "领地名"]
    #[description = "不填则显示用户在所有领地的信息"]
    fief_name: Option<String>,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    // try to create user in db
    let user_id = id_of(ctx.author());
    repo.user().create(user_id, false).await?;

    Ok(())
}

/// 获取用户信息
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn info(
    ctx: Context<'_>,
    #[rename = "用户名"] username: String,

    #[rename = "领地名"]
    #[description = "不填则显示用户在所有领地的信息"]
    fief_name: Option<String>,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    // try to create user in db
    let user_id = id_of(ctx.author());
    repo.user().create(user_id, false).await?;

    Ok(())
}
