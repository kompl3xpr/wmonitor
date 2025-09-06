use poise::{CreateReply, serenity_prelude::CreateAttachment};

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
