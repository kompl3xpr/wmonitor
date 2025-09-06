use super::{Context, Error};

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
pub(super) async fn rename(
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

/// 上传该区块的参考图
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn setref(
    _ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    Ok(())
}

/// 设置该区块的参考图为当前状态
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn refnow(
    _ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    Ok(())
}

/// 上传该区块的遮罩图（用于划定哪些像素需要检查）
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn setmask(
    _ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    Ok(())
}

/// 修改区块的坐标
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn setpos(
    _ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    Ok(())
}

/// 获取区块的信息
#[poise::command(prefix_command, slash_command, category = "区块")]
pub(super) async fn info(
    _ctx: Context<'_>,
    #[rename = "领地名"] fief_name: String,

    #[rename = "区块名"]
    #[description = "区块的原名字"]
    name: String,
) -> Result<(), Error> {
    Ok(())
}
