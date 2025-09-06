use crate::{
    bot::commands::{has_perms, id_of},
    domains::{Permissions, UserId},
};
use poise::serenity_prelude::{Mention, MessageBuilder};

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
    #[rename = "用户"] user: Mention,
    #[rename = "领地名"] fief_name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(fief_id) = repo.fief().id(&fief_name).await else {
        ctx.say(format!("错误：领地 **{fief_name}** 不存在。"))
            .await?;
        return Ok(());
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, Permissions::MEMBER_INVITE).await {
        ctx.say("操作失败，权限不足。").await?;
        return Ok(());
    }

    let Mention::User(user_id) = user else {
        ctx.say(format!("参数错误：请@用户作为输入。")).await?;
        return Ok(());
    };

    let user_id = UserId(user_id.get() as i64);
    if let Err(e) = repo.user().create(user_id, false).await {
        ctx.say(format!("无法存储用户信息: {e}。")).await?;
    }

    match repo.user().join(user_id, fief_id, None).await {
        Ok(true) => {
            ctx.say(format!("已添加用户 {user} 至领地 **{fief_name}**。"))
                .await?
        }
        Ok(false) => {
            ctx.say(format!(
                "错误：用户 {user} 已经是领地 **{fief_name}** 的成员。"
            ))
            .await?
        }
        Err(e) => {
            ctx.say(format!(
                "错误：无法添加用户 {user} 至领地 **{fief_name}**: {e}。"
            ))
            .await?
        }
    };

    Ok(())
}

/// 将用户从领地移出
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn leave(
    ctx: Context<'_>,
    #[rename = "用户"] user: Mention,
    #[rename = "领地名"] fief_name: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(fief_id) = repo.fief().id(&fief_name).await else {
        ctx.say(format!("错误：领地 **{fief_name}** 不存在。"))
            .await?;
        return Ok(());
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, Permissions::MEMBER_KICK).await {
        ctx.say("操作失败，权限不足。").await?;
        return Ok(());
    }

    let Mention::User(user_id) = user else {
        ctx.say(format!("参数错误：请@用户作为输入。")).await?;
        return Ok(());
    };

    let user_id = UserId(user_id.get() as i64);
    if let Err(e) = repo.user().create(user_id, false).await {
        ctx.say(format!("无法存储用户信息: {e}。")).await?;
    }

    match repo.user().leave(user_id, fief_id).await {
        Ok(true) => {
            ctx.say(format!("已将用户 {user} 从领地 **{fief_name}** 中移出。"))
                .await?
        }
        Ok(false) => {
            ctx.say(format!(
                "错误：用户 {user} 不在领地 **{fief_name}** 或已经被移出。"
            ))
            .await?
        }
        Err(e) => {
            ctx.say(format!(
                "错误：无法将用户 {user} 从领地 **{fief_name}** 中移出: {e}。"
            ))
            .await?
        }
    };

    Ok(())
}

/// 给予用户对领地的权限
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn allow(
    ctx: Context<'_>,
    #[rename = "用户"] user: Mention,
    #[rename = "领地名"] fief_name: String,

    #[rename = "权限"]
    #[description = "可以通过 `/wmpermissions` 了解所有权限"]
    permission: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(fief_id) = repo.fief().id(&fief_name).await else {
        ctx.say(format!("错误：领地 **{fief_name}** 不存在。"))
            .await?;
        return Ok(());
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, Permissions::MEMBER_EDIT_PERMS).await {
        ctx.say("操作失败，权限不足。").await?;
        return Ok(());
    }

    let Mention::User(user_id) = user else {
        ctx.say(format!("参数错误：请@用户作为输入。")).await?;
        return Ok(());
    };

    let user_id = UserId(user_id.get() as i64);
    if let Err(e) = repo.user().create(user_id, false).await {
        ctx.say(format!("无法存储用户信息: {e}。")).await?;
    }

    let Ok(perms) = repo.user().permissions_in(user_id, fief_id).await else {
        ctx.say(format!("错误：用户 {user} 并不属于领地 **{fief_name}**。"))
            .await?;
        return Ok(());
    };

    let Some(p) = Permissions::from_name(&permission) else {
        ctx.say(format!("错误：`{permission}` 不是有效的权限名称。"))
            .await?;
        return Ok(());
    };

    if perms.contains(p) {
        ctx.say(format!("错误：用户 {user} 已有权限 `{permission}`。"))
            .await?;
        return Ok(());
    }

    match repo
        .user()
        .set_permissions_in(user_id, fief_id, p | perms)
        .await
    {
        Ok(_) => ctx.say(format!(
            "已经授予用户 {user} 在领地 **{fief_name}** 的 `{permission}` 权限。"
        )),
        Err(e) => ctx.say(format!(
            "错误：无法在领地 **{fief_name}** 为用户 {user} 添加权限: {e}。"
        )),
    }
    .await?;

    Ok(())
}

/// 收回用户对领地的权限
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn deny(
    ctx: Context<'_>,
    #[rename = "用户"] user: Mention,
    #[rename = "领地名"] fief_name: String,
    #[rename = "权限"]
    #[description = "可以通过 `/wmpermissions` 了解所有权限"]
    permission: String,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Ok(fief_id) = repo.fief().id(&fief_name).await else {
        ctx.say(format!("错误：领地 **{fief_name}** 不存在。"))
            .await?;
        return Ok(());
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, Permissions::MEMBER_EDIT_PERMS).await {
        ctx.say("操作失败，权限不足。").await?;
        return Ok(());
    }

    let Mention::User(user_id) = user else {
        ctx.say(format!("参数错误：请@用户作为输入。")).await?;
        return Ok(());
    };

    let user_id = UserId(user_id.get() as i64);
    if let Err(e) = repo.user().create(user_id, false).await {
        ctx.say(format!("无法存储用户信息: {e}。")).await?;
    }

    let Ok(perms) = repo.user().permissions_in(user_id, fief_id).await else {
        ctx.say(format!("错误：用户 {user} 并不属于领地 **{fief_name}**。"))
            .await?;
        return Ok(());
    };

    let Some(p) = Permissions::from_name(&permission) else {
        ctx.say(format!("错误：`{permission}` 不是有效的权限名称。"))
            .await?;
        return Ok(());
    };

    if perms.intersection(p) == Permissions::NONE {
        ctx.say(format!("错误：用户 {user} 未有权限 `{permission}`。"))
            .await?;
        return Ok(());
    }

    match repo
        .user()
        .set_permissions_in(user_id, fief_id, perms - p)
        .await
    {
        Ok(_) => ctx.say(format!(
            "已经收回用户 {user} 在领地 **{fief_name}** 的 `{permission}` 权限。"
        )),
        Err(e) => ctx.say(format!(
            "错误：无法在领地 **{fief_name}** 为用户 {user} 收回权限: {e}。"
        )),
    }
    .await?;

    Ok(())
}

/// 获取用户信息
#[poise::command(prefix_command, slash_command, category = "用户")]
pub(super) async fn info(
    ctx: Context<'_>,
    #[rename = "用户"] user: Mention,

    #[rename = "领地名"]
    #[description = "不填则显示用户在所有领地的信息"]
    fief_name: Option<String>,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;

    let Mention::User(user_id) = user else {
        ctx.say(format!("参数错误：请@用户作为输入。")).await?;
        return Ok(());
    };

    let user_id = UserId(user_id.get() as i64);
    if let Err(e) = repo.user().create(user_id, false).await {
        ctx.say(format!("无法存储用户信息: {e}。")).await?;
    }

    let fief_ids = if let Some(fief_name) = fief_name {
        let Ok(fief_id) = repo.fief().id(&fief_name).await else {
            ctx.say(format!("错误：领地 **{fief_name}** 不存在。"))
                .await?;
            return Ok(());
        };
        vec![fief_id]
    } else {
        repo.user().fiefs(user_id).await?
    };

    let is_admin = repo.user().user_by_id(user_id).await?.is_admin;
    let mut builder = MessageBuilder::new();
    builder
        .push("# 用户基本信息")
        .push(format!("\nDiscord ID：`{}`\n", user_id.0))
        .push(format!(
            "WMonitor 管理员：`{}\n`",
            if is_admin { "是" } else { "否" }
        ))
        .push("\n# 所属领地");

    for fief_id in fief_ids {
        let name = repo.fief().name(fief_id).await?;
        let perms = repo.user().permissions_in(user_id, fief_id).await?;
        let perms_str = perms
            .iter_names()
            .map(|(s, _)| s)
            .fold(String::new(), |a, s| a + "`" + s + "` ");
        builder
            .push(format!("\n- 领地名: **{name}**\n"))
            .push(format!("  拥有权限：{}\n", perms_str));
    }

    ctx.say(builder.build()).await?;
    Ok(())
}
