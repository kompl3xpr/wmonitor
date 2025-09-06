use poise::{
    CreateReply,
    serenity_prelude::{CreateAttachment, MessageBuilder},
};

use crate::{
    Repositories,
    domains::{FiefId, Permissions, UserId},
    net,
};

use super::{Context, Data, Error};

mod admin;
mod chunk;
mod fief;
mod user;

pub(super) fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        wmhelp(),
        wmfetch(),
        wmpermissions(),
        chunk::wmchunk(),
        admin::wmadmin(),
        user::wmuser(),
        fief::wmfief(),
    ]
}

/// 展示所有指令
#[poise::command(prefix_command, track_edits, slash_command, category = "基本指令")]
pub async fn wmhelp(
    ctx: Context<'_>,
    // #[description = "展示 WMonitor 的所有指令"]
    #[autocomplete = "poise::builtins::autocomplete_command"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            show_subcommands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// 根据坐标从 wplace.live 获取区块图片
#[poise::command(prefix_command, track_edits, slash_command, category = "基本指令")]
pub async fn wmfetch(
    ctx: Context<'_>,
    #[rename = "x"]
    #[description = "区块在 Wplace 上的 X 坐标"]
    x: usize,

    #[rename = "y"]
    #[description = "区块在 Wplace 上的 Y 坐标"]
    y: usize,
) -> Result<(), Error> {
    ctx.say("正在从 wplace.live 获取图片，请稍等...").await?;
    let Ok((_, img)) = net::fetch_current_image([x, y]).await else {
        ctx.say("网络异常，请稍后重试。").await?;
        return Ok(());
    };
    let file_name = format!("chunk_{x}_{y}.png");
    ctx.send(
        CreateReply::default().attachment(CreateAttachment::bytes(img.into_inner(), file_name)),
    )
    .await?;
    Ok(())
}

/// 列出用户权限种类
#[poise::command(prefix_command, track_edits, slash_command, category = "基本指令")]
pub async fn wmpermissions(ctx: Context<'_>) -> Result<(), Error> {
    let mut msg = MessageBuilder::new();
    msg.push("# 权限说明\n")
        .push("## 领地相关\n")
        .push("- `FIEF_EDIT`: 编辑领地信息\n")
        .push("- `FIEF_DELETE`: 删除领地\n")
        .push("- `FIEF_ALL`: 领地的全部权限，等同于 `FIEF_EDIT` + `FIEF_DELETE`\n")
        .push("## 区块相关\n")
        .push("- `CHUNK_ADD`: 在领地内添加区块\n")
        .push("- `CHUNK_EDIT`: 编辑领地内区块信息\n")
        .push("- `CHUNK_DELETE`: 删除领地内的区块\n")
        .push("- `CHUNK_ALL`: 区块的全部权限，详细说明同上\n")
        .push("## 成员相关\n")
        .push("- `MEMBER_INVITE`: 邀请成员至领地\n")
        .push("- `MEMBER_EDIT_PERMS`: 编辑用户在领地内的权限\n")
        .push("- `MEMBER_KICK`: 将成员移出领地\n")
        .push("- `MEMBER_ALL`: 成员的全部权限，详细说明同上\n")
        .push("## 其他\n")
        .push("- `NONE`: 无任何权限\n")
        .push("- `ALL`: 拥有上述所有权限\n");
    ctx.say(msg.build()).await?;
    Ok(())
}

fn id_of(user: &poise::serenity_prelude::User) -> UserId {
    UserId(user.id.get() as i64)
}

async fn has_perms(repo: &Repositories, id: UserId, fief_id: FiefId, perms: Permissions) -> bool {
    let Ok(user) = repo.user().user_by_id(id).await else {
        return false;
    };
    if user.is_admin {
        return true;
    }
    let Ok(p) = repo.user().permissions_in(id, fief_id).await else {
        return false;
    };

    perms.intersection(p) == perms
}
