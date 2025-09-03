use crate::repos::traits;
use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{str::FromStr, sync::Arc};

pub struct Repositories {
    user: Box<dyn traits::UserRepo>,
    chunk: Box<dyn traits::ChunkRepo>,
    fief: Box<dyn traits::FiefRepo>,
    event: Box<dyn traits::EventRepo>,
}

impl Repositories {
    pub async fn from_sqlx(url :&str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(&url)?.create_if_missing(true);
        let pool = Arc::new(SqlitePool::connect_with(options).await?);

        sqlx::migrate!("db/migrations").run(&*pool).await?;

        use super::sqlx::*;
        Ok(Self {
            user: Box::new(SqlxUserRepo::new(Arc::clone(&pool))),
            fief: Box::new(SqlxFiefRepo::new(Arc::clone(&pool))),
            chunk: Box::new(SqlxChunkRepo::new(Arc::clone(&pool))),
            event: Box::new(SqlxEventRepo::new(pool)),
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
