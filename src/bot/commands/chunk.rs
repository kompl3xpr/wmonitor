use std::time::Duration;

use poise::serenity_prelude::{EditMessage, MessageBuilder, MessageCollector};

use crate::{
    core::{ImagePng, Position},
    domains::{Chunk, FiefId, Permissions},
    net,
};

use super::{Context, Error, has_perms, id_of};

/// 区块操作
#[poise::command(
    prefix_command,
    slash_command,
    category = "区块",
    subcommands(
        "add", "remove", "rename", "setref", "refnow", "setmask", "setpos", "info"
    )
)]
pub(super) async fn wmchunk(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

async fn _try(
    ctx: Context<'_>,
    fief_name: &String,
    name: &String,
    perms: Permissions,
) -> Result<Option<(FiefId, Chunk)>, Error> {
    let repo = &ctx.data().repo;
    let Ok(fief_id) = repo.fief().id(fief_name).await else {
        ctx.say(format!("错误：领地 **{fief_name}** 不存在。"))
            .await?;
        return Ok(None);
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, perms).await {
        ctx.say("错误：操作失败，权限不足。").await?;
        return Ok(None);
    }

    let Ok(chunk) = repo.chunk().chunk_by_name(fief_id, name).await else {
        ctx.say(format!(
            "错误：无法从领地 **{fief_name}** 中找到区块 *{name}*。"
        ))
        .await?;
        return Ok(None);
    };

    Ok(Some((fief_id, chunk)))
}

/// 为领地添加区块
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn add(
    ctx: Context<'_>,
    #[rename = "领地名"]
    #[description = "区块所在领地的名字"]
    fief_name: String,

    #[rename = "区块名"]
    #[description = "给区块起个名字"]
    name: String,

    #[rename = "x"]
    #[description = "区块在 Wplace 上的 X 坐标"]
    x: usize,

    #[rename = "y"]
    #[description = "区块在 Wplace 上的 Y 坐标"]
    y: usize,
) -> Result<(), Error> {
    let repo = &ctx.data().repo;
    let Ok(fief_id) = repo.fief().id(&fief_name).await else {
        ctx.say(format!("错误：领地 **{fief_name}** 不存在。"))
            .await?;
        return Ok(());
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, Permissions::CHUNK_ADD).await {
        ctx.say("错误：操作失败，权限不足。").await?;
        return Ok(());
    }

    let msg = match repo.chunk().create(&name, fief_id, [x, y].into()).await {
        Ok(Some(id)) => format!(
            "成功在领地 **{fief_name}** 内创建区块 *{name}*(id: `{}`)。",
            id.0
        ),
        Ok(None) => format!("错误：区块 *{name}* 早已存在于领地 **{fief_name}**。"),
        Err(e) => format!("错误：无法在领地 **{fief_name}** 内创建区块 *{name}*: {e}"),
    };

    ctx.say(msg).await?;
    Ok(())
}

/// 为领地删除区块
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn remove(
    ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"] name: String,
) -> Result<(), Error> {
    let Some((_, chunk)) = _try(ctx, &fief_name, &name, Permissions::CHUNK_DELETE).await? else {
        return Ok(());
    };
    let repo = &ctx.data().repo;

    let msg = match repo.chunk().remove_by_id(chunk.id).await {
        Ok(true) => format!("成功将区块 *{name}* 从领地 **{fief_name}** 中删除。"),
        Ok(false) => format!("错误：无法从领地 **{fief_name}** 中找到区块 *{name}*。"),
        Err(e) => format!("错误：无法将区块 *{name}* 从领地 **{fief_name}** 中删除: {e}。"),
    };
    ctx.say(msg).await?;
    Ok(())
}

/// 设置区块名字
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn rename(
    ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,

    #[rename = "新名字"]
    #[description = "给区块起个新名字"]
    new_name: String,
) -> Result<(), Error> {
    let Some((_, chunk)) = _try(ctx, &fief_name, &name, Permissions::CHUNK_EDIT).await? else {
        return Ok(());
    };
    let repo = &ctx.data().repo;

    let msg = match repo.chunk().rename(chunk.id, &new_name).await {
        Ok(_) => format!("成功将领地 **{fief_name}** 内的区块 *{name}* 更名为 *{new_name}*。"),
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    ctx.say(msg).await?;
    Ok(())
}

/// 上传该区块的参考图
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn setref(
    ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    let Some((_, chunk)) = _try(ctx, &fief_name, &name, Permissions::CHUNK_EDIT).await? else {
        return Ok(());
    };
    let repo = &ctx.data().repo;
    ctx.say("请发送图片以上传参考图。").await?.message().await?;

    let mut img: Option<Vec<u8>> = None;
    for _ in 0..3 {
        let Some(mut msg) = MessageCollector::new(ctx)
            .channel_id(ctx.channel_id())
            .author_id(ctx.author().id)
            .timeout(Duration::from_secs(60))
            .await
        else {
            ctx.say("等待已超时，请重新输入指令。").await?;
            return Ok(());
        };

        if msg.attachments.is_empty() {
            ctx.say("找不到附件，请再次上传。").await?;
            continue;
        }
        let file = msg.attachments.remove(0);
        if file.content_type.as_ref().map(|ty| ty == "image/png").unwrap_or(false) {
            img = Some(file.download().await?);
            break;
        } else {
            ctx.say("附件类型只能是 PNG 图片，请再次上传。").await?;
            continue;
        }
    }

    let Some(img) = img.map(ImagePng::new) else {
        ctx.say("错误：失败超过三次，请重新输入指令。").await?;
        return Ok(());
    };

    let msg = match repo.chunk().update_ref_img(chunk.id, Some(img)).await {
        Ok(_) => format!("成功更新领地 **{fief_name}** 内区块 *{name}* 的参考图。"),
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    ctx.say(msg).await?;
    Ok(())
}

/// 上传该区块的遮罩图（用于划定哪些像素需要检查）
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn setmask(
    ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    let Some((_, chunk)) = _try(ctx, &fief_name, &name, Permissions::CHUNK_EDIT).await? else {
        return Ok(());
    };
    let repo = &ctx.data().repo;
    ctx.say("请发送图片以上传遮罩图。").await?.message().await?;

    let mut img: Option<Vec<u8>> = None;
    for _ in 0..3 {
        let Some(mut msg) = MessageCollector::new(ctx)
            .channel_id(ctx.channel_id())
            .author_id(ctx.author().id)
            .timeout(Duration::from_secs(60))
            .await
        else {
            ctx.say("等待已超时，请重新输入指令。").await?;
            return Ok(());
        };

        if msg.attachments.is_empty() {
            ctx.say("找不到附件，请再次上传。").await?;
            continue;
        }
        let file = msg.attachments.remove(0);
        if file.content_type.as_ref().map(|ty| ty == "image/png").unwrap_or(false) {
            img = Some(file.download().await?);
            break;
        } else {
            ctx.say("附件类型只能是 PNG 图片，请再次上传。").await?;
            continue;
        }
    }

    let Some(img) = img.map(ImagePng::new) else {
        ctx.say("错误：失败超过三次，请重新输入指令。").await?;
        return Ok(());
    };

    let msg = match repo.chunk().update_mask_img(chunk.id, Some(img)).await {
        Ok(_) => format!("成功更新领地 **{fief_name}** 内区块 *{name}* 的遮罩图。"),
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    ctx.say(msg).await?;
    Ok(())
}

/// 设置该区块的参考图为当前状态
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn refnow(
    ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    let Some((_, chunk)) = _try(ctx, &fief_name, &name, Permissions::CHUNK_EDIT).await? else {
        return Ok(());
    };
    let repo = &ctx.data().repo;

    ctx.say("正在从 wplace.live 获取图片，请稍等...").await?;
    let Position { x, y } = chunk.position;
    let Ok((_, img)) = net::fetch_current_image([x, y]).await else {
        ctx.say("网络异常，请稍后重试。").await?;
        return Ok(());
    };

    let msg = match repo.chunk().update_ref_img(chunk.id, Some(img)).await {
        Ok(_) => format!("成功将领地 **{fief_name}** 内区块 *{name}* 的参考图更新为当前状态。"),
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    ctx.say(msg).await?;
    Ok(())
}

/// 修改区块的坐标
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn setpos(
    ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,

    #[rename = "x"]
    #[description = "区块在 Wplace 上的 X 坐标"]
    x: usize,

    #[rename = "y"]
    #[description = "区块在 Wplace 上的 Y 坐标"]
    y: usize,
) -> Result<(), Error> {
    let Some((_, chunk)) = _try(ctx, &fief_name, &name, Permissions::CHUNK_EDIT).await? else {
        return Ok(());
    };
    let repo = &ctx.data().repo;

    let msg = match repo.chunk().set_position(chunk.id, [x, y].into()).await {
        Ok(_) => format!("成功将领地 **{fief_name}** 内的区块 *{name}* 坐标改为 `({x}, {y})`。"),
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    ctx.say(msg).await?;
    Ok(())
}

/// 获取区块的信息
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn info(
    ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    let Some((_, chunk)) = _try(ctx, &fief_name, &name, Permissions::NONE).await? else {
        return Ok(());
    };
    let _ = &ctx.data().repo;

    ctx.say(format!("{chunk:#?}")).await?;
    Ok(())
}
