use std::time::Duration;

use poise::{
    CreateReply,
    serenity_prelude::{CreateAttachment, MessageBuilder, MessageCollector},
};

use super::{Context, Error, has_perms, id_of, say};
use crate::{
    core::{ImagePng, Position},
    domains::{Chunk, FiefId, Permissions},
    net,
};

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
        say!(ctx, "错误：领地 **{fief_name}** 不存在。");
        return Ok(None);
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, perms).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(None);
    }

    let Ok(chunk) = repo.chunk().chunk_by_name(fief_id, name).await else {
        say!(
            ctx,
            "错误：无法从领地 **{fief_name}** 中找到区块 *{name}*。"
        );
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
        say!(ctx, "错误：领地 **{fief_name}** 不存在。");
        return Ok(());
    };

    let author_id = id_of(ctx.author());
    if !has_perms(repo, author_id, fief_id, Permissions::CHUNK_ADD).await {
        say!(ctx, "错误：操作失败，权限不足。");
        return Ok(());
    }

    let msg = match repo.chunk().create(&name, fief_id, [x, y].into()).await {
        Ok(Some(id)) => format!(
            "成功在领地 **{fief_name}** 内创建区块 *{name}*(id: `{}`)。",
            id.0
        ),
        Ok(None) => {
            format!("错误：区块 *{name}* 早已存在于领地 **{fief_name}**。")
        }
        Err(e) => {
            format!("错误：无法在领地 **{fief_name}** 内创建区块 *{name}*: {e}")
        }
    };

    say!(ctx, msg);
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
        Ok(true) => {
            format!("成功将区块 *{name}* 从领地 **{fief_name}** 中删除。")
        }
        Ok(false) => {
            format!("错误：无法从领地 **{fief_name}** 中找到区块 *{name}*。")
        }
        Err(e) => format!("错误：无法将区块 *{name}* 从领地 **{fief_name}** 中删除: {e}。"),
    };
    say!(ctx, msg);
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
    say!(ctx, msg);
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
    say!(ctx, "请发送图片以上传参考图。").message().await?;

    let mut img: Option<Vec<u8>> = None;
    for _ in 0..3 {
        let Some(mut msg) = MessageCollector::new(ctx)
            .channel_id(ctx.channel_id())
            .author_id(ctx.author().id)
            .timeout(Duration::from_secs(60))
            .await
        else {
            say!(ctx, "等待已超时，请重新输入指令。");
            return Ok(());
        };

        if msg.attachments.is_empty() {
            say!(ctx, "找不到附件，请再次上传。");
            msg.delete(ctx).await?;
            continue;
        }
        let file = msg.attachments.remove(0);
        if file
            .content_type
            .as_ref()
            .map(|ty| ty == "image/png")
            .unwrap_or(false)
        {
            img = Some(file.download().await?);
            msg.delete(ctx).await?;
            break;
        } else {
            say!(ctx, "附件类型只能是 PNG 图片，请再次上传。");
            msg.delete(ctx).await?;
        }
    }

    let Some(img) = img.map(ImagePng::new) else {
        say!(ctx, "错误：失败超过三次，请重新输入指令。");
        return Ok(());
    };

    let msg = match repo.chunk().update_ref_img(chunk.id, Some(img)).await {
        Ok(_) => {
            format!("成功更新领地 **{fief_name}** 内区块 *{name}* 的参考图。")
        }
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    say!(ctx, msg);
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
    say!(ctx, "请发送图片以上传遮罩图。").message().await?;

    let mut img: Option<Vec<u8>> = None;
    for _ in 0..3 {
        let Some(mut msg) = MessageCollector::new(ctx)
            .channel_id(ctx.channel_id())
            .author_id(ctx.author().id)
            .timeout(Duration::from_secs(60))
            .await
        else {
            say!(ctx, "等待已超时，请重新输入指令。");
            return Ok(());
        };

        if msg.attachments.is_empty() {
            say!(ctx, "找不到附件，请再次上传。");
            msg.delete(ctx).await?;
            continue;
        }
        let file = msg.attachments.remove(0);
        if file
            .content_type
            .as_ref()
            .map(|ty| ty == "image/png")
            .unwrap_or(false)
        {
            img = Some(file.download().await?);
            msg.delete(ctx).await?;
            break;
        } else {
            say!(ctx, "附件类型只能是 PNG 图片，请再次上传。");
            msg.delete(ctx).await?;
        }
    }

    let Some(img) = img.map(ImagePng::new) else {
        say!(ctx, "错误：失败超过三次，请重新输入指令。");
        return Ok(());
    };

    let msg = match repo.chunk().update_mask_img(chunk.id, Some(img)).await {
        Ok(_) => {
            format!("成功更新领地 **{fief_name}** 内区块 *{name}* 的遮罩图。")
        }
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    say!(ctx, msg);
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

    say!(ctx, "正在从 wplace.live 获取图片，请稍等……");
    let Position { x, y } = chunk.position;
    let Ok((_, img)) = net::fetch_current_image([x, y]).await else {
        say!(ctx, "网络异常，请稍后重试。");
        return Ok(());
    };

    let msg = match repo.chunk().update_ref_img(chunk.id, Some(img)).await {
        Ok(_) => format!("成功将领地 **{fief_name}** 内区块 *{name}* 的参考图更新为当前状态。"),
        Err(e) => format!("错误：无法修改领地 **{fief_name}** 内的区块 *{name}*: {e}。"),
    };
    say!(ctx, msg);
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
    say!(ctx, msg);
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
    let repo = &ctx.data().repo;
    let mut builder = MessageBuilder::new();
    builder
        .push("# 区块信息\n")
        .push("区块名：*")
        .push(chunk.name)
        .push("*\n属于领地：**")
        .push(fief_name)
        .push("**\n位置：")
        .push("(`")
        .push(chunk.position.x.to_string())
        .push("`, `")
        .push(chunk.position.y.to_string())
        .push("`)\n");

    let ref_ = repo.chunk().ref_img(chunk.id).await?;
    builder.push("参考图：").push(if ref_.is_none() {
        ":negative_squared_cross_mark: 未设置\n"
    } else {
        ":white_check_mark: 已设置\n"
    });

    let mask = repo.chunk().mask_img(chunk.id).await?;
    builder.push("遮罩图：").push(if mask.is_none() {
        ":negative_squared_cross_mark: 未设置\n"
    } else {
        ":white_check_mark: 已设置\n"
    });

    let result = repo.chunk().result_img(chunk.id).await?;
    let mut reply = CreateReply::default()
        .content(builder.build())
        .ephemeral(true);

    if let Some(result) = result {
        reply = reply.attachment(CreateAttachment::bytes(result.into_inner(), "status.png"));
    }

    ctx.send(reply).await?;
    Ok(())
}
