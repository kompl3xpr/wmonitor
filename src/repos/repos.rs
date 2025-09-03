use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use tracing::{error, info};
use std::{str::FromStr, sync::Arc};
use crate::repos::traits;

pub struct Repositories {
    user: Box<dyn traits::UserRepo>,
    chunk: Box<dyn traits::ChunkRepo>,
    fief: Box<dyn traits::FiefRepo>,
    event: Box<dyn traits::EventRepo>,
}

impl Repositories {
    pub async fn from_sqlx() -> Result<Self> {
        let pool = {
            let url = &crate::cfg().common.database_url;
            let options = if url.is_empty() {
                info!("attempting to use environment variable `DATABASE_URL`...");
                let url = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
                    error!("failed to get variable `DATABASE_URL`: {e}");
                    panic!();
                });
                SqliteConnectOptions::from_str(&url)?.create_if_missing(true)
            } else {
                SqliteConnectOptions::from_str(&url)?.create_if_missing(true)
            };
            Arc::new(SqlitePool::connect_with(options).await?)
        };

        sqlx::migrate!("db/migrations").run(&*pool).await?;

        use super::sqlx::*;
        let user = Box::new(SqlxUserRepo::new(Arc::clone(&pool)));
        let fief = Box::new(SqlxFiefRepo::new(Arc::clone(&pool)));
        let chunk = Box::new(SqlxChunkRepo::new(Arc::clone(&pool)));
        let event = Box::new(SqlxEventRepo::new(pool));
        Ok(Self {
            user,
            fief,
            chunk,
            event,
        })
    }

    pub fn user(&self) -> &dyn traits::UserRepo {
        &*self.user
    }

    pub fn fief(&self) -> &dyn traits::FiefRepo {
        &*self.fief
    }

    pub fn chunk(&self) -> &dyn traits::ChunkRepo {
        &*self.chunk
    }

    pub fn event(&self) -> &dyn traits::EventRepo {
        &*self.event
    }
}