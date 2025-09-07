use super::Error;
use crate::{Repositories, check::Event};
use poise::serenity_prelude::{CreateAttachment, CreateMessage, Mention, MessageBuilder};

pub async fn notification_message(
    repo: &Repositories,
    event: Event,
) -> Result<CreateMessage, Error> {
    let result = CreateMessage::new();

    Ok(match event {
        Event::DiffFound(fief_id, chunk_ids) => {
            let users = repo.fief().members(fief_id).await?;
            let mentions = users
                .into_iter()
                .map(|u| Mention::User((u.0 as u64).into()))
                .fold(String::new(), |s, m| s + m.to_string().as_str() + " ");

            let name = repo.fief().name(fief_id).await?;
            let diff_count = repo.fief().diff_count(fief_id).await?;

            let mut chunk_result_imgs = vec![];
            let mut chunk_names = String::new();
            for id in chunk_ids {
                let name = repo.chunk().name(id).await?;
                let img = repo.chunk().result_img(id).await?;
                chunk_names = chunk_names + "*" + name.as_str() + "* ";
                chunk_result_imgs.push(img);
            }

            let content = MessageBuilder::new()
                .push("# 发现异常像素\n")
                .push(format!("领地: **{name}**\n"))
                .push(format!("异常区块：{chunk_names}\n"))
                .push(format!("异常像素数量：{diff_count} 个\n"))
                .push(mentions)
                .build();
            result.content(content).add_files(
                chunk_result_imgs
                    .into_iter()
                    .filter_map(|img| img)
                    .enumerate()
                    .map(|(i, img)| {
                        CreateAttachment::bytes(img.into_inner(), format!("diff_{i}.png"))
                    }),
            )
        }

        Event::NetworkError(e) => result.content(format!("网络异常：{e}。")),

        Event::ChunkRefMissing(fief_id, chunk_id) => {
            let f = repo.fief().name(fief_id).await?;
            let c = repo.chunk().name(chunk_id).await?;
            result.content(format!("警告：领地 **{f}** 的区块 *{c}* 未设置参考图。"))
        }

        Event::ChunkMaskMissing(fief_id, chunk_id) => {
            let f = repo.fief().name(fief_id).await?;
            let c = repo.chunk().name(chunk_id).await?;
            result.content(format!("警告：领地 **{f}** 的区块 *{c}* 未设置遮罩图。"))
        }
    })
}
