use std::sync::atomic::Ordering;

use poise::serenity_prelude::{Http, Mention, MessageBuilder};

use super::{Context, Error, say};
use crate::{
    bot::commands::{id_of, start_with},
    domains::{User, UserId},
};

/// 管理员指令
#[poise::command(
    prefix_command,
    slash_command,
    category = "管理员",
    subcommands("op", "deop", "listop", "stop", "start", "fiefs")
)]
pub(super) async fn wmop(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// 启用 WMonitor
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn start(ctx: Context<'_>) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let author = repo.user().user_by_id(id_of(ctx.author())).await;
    let Ok(User { is_admin: true, .. }) = author else {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    };

    let channel = ctx.channel_id();
    let Some(tx) = ctx.data().event_rx.lock().await.take() else {
        say!(ctx, "通知频道设置后无法更改，请重启程序并再次设置。");
        return Ok(());
    };
    let http = Http::new(ctx.http().token());
    start_with(http, ctx.data().repo, tx, channel).await
}

/// 关闭 WMonitor
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let author = repo.user().user_by_id(id_of(ctx.author())).await;
    let Ok(User { is_admin: true, .. }) = author else {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    };

    ctx.data().should_close.store(true, Ordering::SeqCst);
    say!(
        ctx,
        "已发送关闭信号，请等待检查器完成工作（大概需要一分钟）。"
    );
    Ok(())
}

/// 显示所有管理员
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn listop(ctx: Context<'_>) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let author = repo.user().user_by_id(id_of(ctx.author())).await;
    let Ok(User { is_admin: true, .. }) = author else {
        say!(ctx, "错误：操作失败，权限不足。");
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
            say!(ctx, mentions)
        }
        Err(e) => say!(ctx, "错误：无法获取管理员列表: {e}"),
    };
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
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    };

    let Mention::User(user_id) = user else {
        say!(ctx, "参数错误：请@用户作为输入。");
        return Ok(());
    };

    let user_id = UserId(user_id.get() as i64);
    if let Err(e) = repo.user().create(user_id, false).await {
        say!(ctx, "无法存储用户信息: {e}。");
    }

    let a = if is_admin { "" } else { "非" };
    let is_admin_old = repo.user().user_by_id(user_id).await?.is_admin;
    if is_admin_old == is_admin {
        say!(ctx, "错误：{user} 已经是{a}管理员。");
        return Ok(());
    }

    match repo.user().set_admin(user_id, is_admin).await {
        Ok(_) => say!(ctx, "已设置 {user} 为{a}管理员。"),
        Err(e) => say!(ctx, "错误：无法设置 {user} 为{a}管理员: {e}。"),
    };
    Ok(())
}

/// 列出所有领地
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn fiefs(ctx: Context<'_>) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let author = repo.user().user_by_id(id_of(ctx.author())).await;
    let Ok(User { is_admin: true, .. }) = author else {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    };

    let Ok(fiefs) = repo.fief().all().await else {
        say!(ctx, "错误：无法获取领地信息。");
        return Ok(());
    };

    let mut builder = MessageBuilder::new();
    builder.push("# 领地列表\n");
    let now = chrono::Utc::now();
    for fief in fiefs {
        let enabled = fief.skip_check_until < now;

        builder.push("`[").push(fief.name).push("] ");
        builder.push(if enabled {
            "启用中 | "
        } else {
            "禁用中 | "
        });

        let chunks = repo.fief().chunk_count(fief.id).await.unwrap_or(0);
        builder.push("区块: ").push(chunks.to_string()).push(" | ");
        let min = fief.check_interval.num_minutes().to_string();
        builder.push("间隔: ").push(min).push(" 分钟`\n");
        let members = repo.fief().members(fief.id).await.unwrap_or(vec![]);
        let mentions = members
            .into_iter()
            .map(|u| Mention::User((u.0 as u64).into()))
            .fold("`成员:` ".to_string(), |s, m| {
                s + m.to_string().as_str() + " "
            });

        builder.push(mentions);
        builder.push("\n\n");
    }

    say!(ctx, builder.build());
    Ok(())
}
