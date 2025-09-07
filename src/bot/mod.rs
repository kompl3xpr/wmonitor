#![warn(clippy::str_to_string)]

mod commands;
mod notification;

use poise::serenity_prelude as serenity;
use tokio::sync::{Mutex, mpsc::Receiver};
use std::{
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};
use tracing::{error, info, warn};

use crate::check::Event;

// Types used by all command functions
type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    pub repo: crate::Repositories,
    pub event_rx: Mutex<Option<Receiver<Event>>>,
    pub should_close: Arc<AtomicBool>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            error!("Failed to start bot: {:?}", error);
            panic!();
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            warn!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                warn!("Error while handling error: {}", e)
            }
        }
    }
}

pub async fn new_client(
    token: &impl AsRef<str>,
    data: Data,
) -> anyhow::Result<serenity::Client> {
    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: commands::all(),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(".".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .options(options)
        .build();

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    Ok(client)
}
