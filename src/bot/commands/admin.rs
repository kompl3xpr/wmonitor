use super::{Context, Error};

/// 管理员指令
#[poise::command(
    prefix_command,
    slash_command,
    category = "管理员",
    subcommands("start", "stop", "op", "deop")
)]
pub(super) async fn wmadmin(_: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 开启 WMonitor
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn start(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 关闭 WMonitor
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn stop(_ctx: Context<'_>) -> Result<(), Error> {
    todo!()
}

/// 添加管理员
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn op(
    _ctx: Context<'_>,

    #[rename = "用户名"] username: String,
) -> Result<(), Error> {
    todo!()
}

/// 取消管理员
#[poise::command(prefix_command, slash_command, category = "管理员")]
pub(super) async fn deop(
    _ctx: Context<'_>,

    #[rename = "用户名"] username: String,
) -> Result<(), Error> {
    todo!()
}
