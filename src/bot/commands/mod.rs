use poise::{serenity_prelude::CreateAttachment, CreateReply};

use crate::net;

use super::{Context, Data, Error};

mod admin;
mod chunk;
mod fief;
mod user;

pub(super) fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        wmhelp(),
        wmfetch(),
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

    let (_, img) = net::fetch_current_image([x, y]).await?;
    let file_name = format!("chunk_{x}_{y}.png");
    ctx.send(CreateReply::default().attachment(
        CreateAttachment::bytes(img.into_inner(), file_name)
    )).await?;
    Ok(())
}
