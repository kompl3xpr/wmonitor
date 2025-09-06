use super::{Context, Error};

/// 区块操作
#[poise::command(
    prefix_command,
    slash_command,
    category = "区块",
    subcommands("add", "remove", "setname")
)]
pub(super) async fn wmchunk(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// 为领地添加区块
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn add(
    _ctx: Context<'_>,
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
    Ok(())
}

/// 为领地删除区块
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn remove(
    _ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"] name: String,
) -> Result<(), Error> {
    Ok(())
}

/// 设置区块名字
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn setname(
    _ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,

    #[rename = "新名字"]
    #[description = "给区块起个新名字"]
    new_name: String,
) -> Result<(), Error> {
    Ok(())
}
