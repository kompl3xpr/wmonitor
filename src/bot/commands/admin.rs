use std::sync::atomic::Ordering;

use poise::serenity_prelude::{Http, Mention};

use crate::{
    bot::{commands::id_of, notification::notification_message},
    domains::{User, UserId},
};

use super::{Context, Error};

/// 管理员指令
#[poise::command(
    prefix_command,
    slash_command,
    category = "管理员",
    subcommands("role", "derole", "listrole", "op", "deop", "listop", "stop", "here")
)]
pub(super) async fn wmadmin(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// 让 WMonitor 在这里发送通知
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn here(ctx: Context<'_>) -> Result<(), Error> {
    let channel = ctx.channel_id();
    let http = Http::new(ctx.http().token());
    let Some(mut tx) = ctx.data().event_rx.lock().await.take() else {
        ctx.say("通知频道设置后无法更改，请重启程序并再次设置。")
            .await?;
        return Ok(());
    };
    let repo = ctx.data().repo.clone();

    tokio::spawn(async move {
        while let Some(event) = tx.recv().await {
            if let Ok(msg) = notification_message(&repo, event).await {
                channel.send_message(&http, msg).await.ok();
            }
        }
    });

    ctx.say("已设置当前频道为通知频道。").await?;
    Ok(())
}

/// 关闭 WMonitor
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data().should_close.store(true, Ordering::SeqCst);
    Ok(())
}

/// 添加可以使用 WMonitor 的身份组
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn role(
    _ctx: Context<'_>,
    #[rename = "身份组"] _role: Mention,
) -> Result<(), Error> {
    todo!()
}

/// 删除可以使用 WMonitor 的身份组
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn derole(
    _ctx: Context<'_>,
    #[rename = "身份组"] _role: Mention,
) -> Result<(), Error> {
    todo!()
}

/// 显示所有权限组
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn listrole(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 显示所有管理员
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn listop(ctx: Context<'_>) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let author = repo.user().user_by_id(id_of(ctx.author())).await;
    let Ok(User { is_admin: true, .. }) = author else {
        ctx.say("错误：操作失败，权限不足。").await?;
        return Ok(());
    };

    match repo.user().admins().await {
        Ok(admins) => {
            let mentions = admins
                .into_iter()
                .map(|u| Mention::User((u.id.0 as u64).into()))
                .fold("# 管理员列表\n".to_string(), |s, m| {
                    s + m.to_string().as_ref() + "\n"
                });
            ctx.say(mentions).await?;
        }
        Err(e) => {
            ctx.say(format!("错误：无法获取管理员列表: {e}")).await?;
        }
    }
    Ok(())
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
        ctx.say("错误：操作失败，权限不足。").await?;
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
