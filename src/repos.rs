use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{str::FromStr, sync::Arc};
pub mod sqlx_repos;

mod chunk;
mod fief;
mod user;

pub mod domains {
    pub use super::chunk::domains::*;
    pub use super::fief::domains::*;
    pub use super::user::domains::*;
}

pub mod traits {
    pub use super::chunk::ChunkRepo;
    pub use super::fief::FiefRepo;
    pub use super::user::UserRepo;
}

pub struct Repositories {
    user: Box<dyn traits::UserRepo>,
    chunk: Box<dyn traits::ChunkRepo>,
    fief: Box<dyn traits::FiefRepo>,
}

impl Repositories {
    pub async fn from_sqlx(url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(url)?.create_if_missing(true);
        let pool = Arc::new(SqlitePool::connect_with(options).await?);

        sqlx::migrate!("db/migrations").run(&*pool).await?;

        use sqlx_repos::*;
        Ok(Self {
            user: Box::new(SqlxUserRepo::new(Arc::clone(&pool))),
            fief: Box::new(SqlxFiefRepo::new(Arc::clone(&pool))),
            chunk: Box::new(SqlxChunkRepo::new(Arc::clone(&pool))),
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
}
