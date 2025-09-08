pub use tracing::{error, info, level_filters::LevelFilter, warn};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt};

pub fn init_logger() -> WorkerGuard {
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::HOURLY)
        .filename_prefix("wmonitor")
        .filename_suffix("log")
        .max_log_files(24)
        .build("logs/")
        .unwrap();
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::Registry::default()
        .with(
            fmt::Layer::new()
                .with_writer(std::io::stderr)
                .with_ansi(true)
                .pretty()
                .with_filter(LevelFilter::INFO),
        )
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_filter(LevelFilter::INFO),
        );

    tracing::subscriber::set_global_default(subscriber).unwrap();
    return guard;
}
