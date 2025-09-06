use poise::serenity_prelude::Mention;

use crate::{
    bot::commands::id_of,
    domains::{User, UserId},
};

use super::{Context, Error};

/// 管理员指令
#[poise::command(
    prefix_command,
    slash_command,
    category = "管理员",
    subcommands("role", "derole", "listrole", "op", "deop", "listop")
)]
pub(super) async fn wmadmin(_: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 添加可以使用 WMonitor 的身份组
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn role(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 删除可以使用 WMonitor 的身份组
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn derole(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 显示所有权限组
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn listrole(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 显示所有管理员
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn listop(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 添加管理员
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn op(ctx: Context<'_>, #[rename = "用户"] user: Mention) -> Result<(), Error> {
    set_admin(ctx, user, true).await
}

/// 取消管理员
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn deop(
    ctx: Context<'_>, #[rename = "用户"] user: Mention
) -> Result<(), Error> {
    set_admin(ctx, user, false).await
}

async fn set_admin(ctx: Context<'_>, user: Mention, is_admin: bool) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let author = repo.user().user_by_id(id_of(ctx.author())).await;
    let Ok(User { is_admin: true, .. }) = author else {
        ctx.say("操作失败，权限不足。").await?;
        return Ok(());
    };

    let Mention::User(user_id) = user else {
        ctx.say(format!("参数错误：请@用户作为输入。")).await?;
        return Ok(());
    };

    let user_id = UserId(user_id.get() as i64);
    if let Err(e) = repo.user().create(user_id, false).await {
        ctx.say(format!("无法存储用户信息: {e}。")).await?;
    }

    let a = if is_admin { "" } else { "非" };
    let is_admin_old = repo.user().user_by_id(user_id).await.unwrap().is_admin;
    if is_admin_old == is_admin {
        ctx.say(format!("错误：{user} 已经是{a}管理员。")).await?;
        return Ok(());
    }

    match repo.user().set_admin(user_id, is_admin).await {
        Ok(_) => ctx.say(format!("已设置 {user} 为{a}管理员。")).await?,
        Err(e) => {
            ctx.say(format!("错误：无法设置 {user} 为{a}管理员: {e}。"))
                .await?
        }
    };
    Ok(())
}
