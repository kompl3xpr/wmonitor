use poise::{
    CreateReply,
    serenity_prelude::{Mention, MessageBuilder},
};

use super::{Context, Error, say};
use crate::{
    bot::commands::{has_perms, id_of},
    domains::Permissions,
};

/// 领地操作
#[poise::command(
    prefix_command,
    slash_command,
    category = "领地",
    subcommands(
        "add", "remove", "check", "rename", "settime", "enable", "disable", "info"
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

    let user_id = id_of(ctx.author());
    if let Err(e) = repo.user().create(user_id, false).await {
        say!(ctx, "错误：无法存储用户信息: {e}。");
    }

    let Some(id) = repo.fief().create(&name, None).await? else {
        let msg = format!("领地 **{name}** 早已存在，请换个名字重新创建。");
        say!(ctx, msg);
        return Ok(());
    };

    repo.user()
        .join(user_id, id, Some(Permissions::ALL))
        .await?;

    say!(ctx, "成功创建领地 **{name}**（id: `{}`）。", id.0);
    Ok(())
}

/// 删除领地
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn remove(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(id) = repo.fief().id(&name).await else {
        say!(ctx, "错误：领地 **{name}** 不存在。");
        return Ok(());
    };

    let user_id = id_of(ctx.author());
    if !has_perms(repo, user_id, id, Permissions::FIEF_DELETE).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    }

    repo.fief().remove_by_name(&name).await?;
    say!(ctx, "成功删除领地 **{name}**。");
    Ok(())
}

/// 设置领地名
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn rename(
    ctx: Context<'_>,
    #[rename = "领地名"]
    #[description = "领地的原名字"]
    name: String,

    #[rename = "新名字"]
    #[description = "给领地起个新名字"]
    new_name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(id) = repo.fief().id(&name).await else {
        say!(ctx, "错误：领地 **{name}** 不存在。");
        return Ok(());
    };
    let user_id = id_of(ctx.author());
    if !has_perms(repo, user_id, id, Permissions::FIEF_EDIT).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    }

    repo.fief().rename(id, &new_name).await?;
    say!(ctx, "已将领地 **{name}** 的名字变更为 **{new_name}**。");
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

    let Ok(id) = repo.fief().id(&name).await else {
        say!(ctx, "错误：领地 **{name}** 不存在。");
        return Ok(());
    };
    let user_id = id_of(ctx.author());
    if !has_perms(repo, user_id, id, Permissions::FIEF_EDIT).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    }

    repo.fief()
        .set_check_interval(id, chrono::Duration::minutes(interval as i64))
        .await?;
    say!(
        ctx,
        "已变更 **{name}** 领地的检查间隔时间为 {interval} 分钟。"
    );
    Ok(())
}

/// 手动检查领地
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn check(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(id) = repo.fief().id(&name).await else {
        say!(ctx, "错误：领地 **{name}** 不存在。");
        return Ok(());
    };
    let user_id = id_of(ctx.author());
    if !has_perms(repo, user_id, id, Permissions::FIEF_EDIT).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    }

    match repo.fief().mark_should_check_now(id).await {
        Ok(_) => {
            say!(ctx, "设置成功，领地 **{name}** 将在一分钟内被执行检查。")
        }
        Err(_) => say!(ctx, "设置失败。"),
    };

    Ok(())
}

/// 启动对领地的定期自动检查（创建领地时自动启用）
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn enable(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(id) = repo.fief().id(&name).await else {
        say!(ctx, "错误：领地 **{name}** 不存在。");
        return Ok(());
    };
    let user_id = id_of(ctx.author());
    if !has_perms(repo, user_id, id, Permissions::FIEF_EDIT).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    }

    repo.fief().keep_check(id).await?;
    say!(ctx, "已启用对领地 **{name}** 的定期自动检查。");
    Ok(())
}

/// 禁用对领地的定期自动检查
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn disable(
    ctx: Context<'_>,
    #[rename = "领地名"] name: String,
    #[rename = "禁用时长"]
    #[description = "多少小时后重新启用"]
    dur_hours: Option<usize>,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(id) = repo.fief().id(&name).await else {
        say!(ctx, "错误：领地 **{name}** 不存在。");
        return Ok(());
    };
    let user_id = id_of(ctx.author());
    if !has_perms(repo, user_id, id, Permissions::FIEF_EDIT).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    }

    match dur_hours {
        Some(d) => {
            let dur = chrono::Duration::hours(d as i64);
            repo.fief().skip_check_for(id, dur, None).await?;
            say!(
                ctx,
                "已禁用对领地 **{name}** 的定期自动检查（持续时间: {d} 小时）。",
            );
        }
        _ => {
            repo.fief().skip_check(id).await?;
            say!(ctx, "已禁用对领地 **{name}** 的定期自动检查。");
        }
    }
    Ok(())
}

/// 获取领地信息
#[poise::command(prefix_command, slash_command, category = "领地")]
pub(super) async fn info(
    ctx: Context<'_>, #[rename = "领地名"] name: String
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(fief) = repo.fief().fief_by_name(&name).await else {
        say!(ctx, "错误：领地 **{name}** 不存在。");
        return Ok(());
    };

    let mut builder = MessageBuilder::new();

    let now = chrono::Utc::now();
    let last_check = now - fief.last_check;
    let last_check = if last_check > chrono::Duration::weeks(100) {
        "无".to_string()
    } else {
        format!("{} 分钟之前", last_check.num_minutes())
    };

    let skip_check_until = if fief.skip_check_until < now {
        ":white_check_mark: 启用中".to_string()
    } else {
        let skip_check_until = fief.skip_check_until - now;
        if skip_check_until > chrono::Duration::weeks(100) {
            ":negative_squared_cross_mark: 禁用中".to_string()
        } else {
            format!(
                ":negative_squared_cross_mark: 禁用中，{} 分钟之后启用",
                skip_check_until.num_minutes()
            )
        }
    };

    builder
        .push("# 领地信息")
        .push("\n名字：**")
        .push(fief.name)
        .push("**\n上次检查：")
        .push(last_check)
        .push("\n定期自动检查：")
        .push(skip_check_until)
        .push("\n定期自动检查间隔：")
        .push(fief.check_interval.num_minutes().to_string())
        .push(" 分钟一次\n");

    let mut chunks = vec![];
    for chunk_id in repo.fief().chunks(fief.id).await? {
        let Ok(chunk) = repo.chunk().chunk_by_id(chunk_id).await else {
            continue;
        };
        chunks.push((chunk.name, chunk.position));
    }

    if !chunks.is_empty() {
        builder.push("# 领地的区块信息\n");
        for (name, pos) in chunks {
            builder.push(format!(
                "- 区块名：*{name}*\n  位置：`({}, {})`\n",
                pos.x, pos.y
            ));
        }
    }

    let mut members = vec![];
    for member_id in repo.fief().members(fief.id).await? {
        let perms = repo.user().permissions_in(member_id, fief.id).await?;
        let perms_str = perms
            .iter_names()
            .map(|(s, _)| s)
            .fold(String::new(), |a, s| a + "`" + s + "` ");
        members.push((
            Mention::User((member_id.0 as u64).into()),
            if perms_str == "" {
                "无".into()
            } else {
                perms_str
            },
        ));
    }

    if !members.is_empty() {
        builder.push("# 领地成员\n");
        for (name, perms) in members {
            builder.push(format!("- 用户：{name}\n  权限：{perms}\n"));
        }
    }

    ctx.send(
        CreateReply::default()
            .content(builder.build())
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
