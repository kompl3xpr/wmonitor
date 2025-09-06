use super::{Context, Error};
/// 领地操作
#[poise::command(
    prefix_command,
    slash_command,
    category = "领地",
    subcommands(
        "add", "remove", "check", "setname", "settime", "enable", "disable", "info"
    )
)]
pub(super) async fn wmfief(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// 添加领地
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn add(
    ctx: Context<'_>,
    #[rename = "领地名"]
    #[description = "给领地起个名字"]
    name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let id = repo.fief().create(&name, None).await?;

    ctx.say(format!("fief_id: {:?}", id)).await?;
    Ok(())
}

/// 删除领地
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn remove(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let result = repo.fief().remove_by_name(&name).await?;
    ctx.say(format!("{:?}", result)).await?;
    Ok(())
}

/// 设置领地名
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn setname(
    ctx: Context<'_>,
    #[rename = "领地名"]
    #[description = "领地的原名字"]
    name: String,

    #[rename = "新名字"]
    #[description = "给领地起个新名字"]
    new_name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let id = repo.fief().id(&name).await?;
    let result = repo.fief().set_name(id, &new_name).await?;
    ctx.say(format!("fief: {:?}", result)).await?;
    Ok(())
}

/// 设置领地检查间隔
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn settime(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,

    #[rename = "间隔"]
    #[description = "检查间隔（分钟）"]
    interval: usize,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let id = repo.fief().id(&name).await?;
    repo.fief().set_check_interval(id, chrono::Duration::minutes(interval as i64)).await?;
    ctx.say(format!("changed")).await?;
    Ok(())
}

/// 手动检查领地
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn check(
    _ctx: Context<'_>,
    #[rename = "领地名"] _name: String,
) -> Result<(), Error> {
    Ok(())
}

/// 启动对领地的自动检查（创建领地时自动启用）
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn enable(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let id = repo.fief().id(&name).await?;
    repo.fief().keep_check(id).await?;
    ctx.say(format!("enabled")).await?;
    Ok(())
}

/// 禁用对领地的自动检查
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn disable(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let id = repo.fief().id(&name).await?;
    repo.fief().skip_check(id).await?;
    ctx.say(format!("disabled")).await?;
    Ok(())
}

/// 获取领地信息
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn info(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let fief = repo.fief().fief_by_name(&name).await?;
    ctx.say(format!("fief: {:?}", fief)).await?;
    Ok(())
}
