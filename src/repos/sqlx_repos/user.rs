use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;

use crate::{
    domains::{FiefId, Permissions, User, UserId},
    entities,
    repos::traits::UserRepo,
};

pub struct SqlxUserRepo(Arc<SqlitePool>);

impl SqlxUserRepo {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self(pool)
    }
}

#[allow(unused)]
#[async_trait]
impl UserRepo for SqlxUserRepo {
    // [C] Create
    async fn create(&self, id: UserId, is_admin: bool) -> Result<Option<UserId>> {
        let result = sqlx::query("INSERT INTO Users (id, is_admin) VALUES ($1, $2)")
            .bind(id.0)
            .bind(is_admin)
            .execute(&*self.0)
            .await;
        Ok(super::conv_create_result(result)?)
    }

    async fn join(&self, id: UserId, fief_id: FiefId, p: Option<Permissions>) -> Result<bool> {
        let permissions = p.unwrap_or(Permissions::NONE);
        let result =
            sqlx::query("INSERT INTO Members (user_id, fief_id, permissions) VALUES ($1, $2, $3)")
                .bind(id.0)
                .bind(fief_id.0)
                .bind(permissions.bits())
                .execute(&*self.0)
                .await;
        Ok(super::conv_create_result::<i64>(result)?.is_some())
    }

    // [R] Read
    // - self or fields
    async fn user_by_id(&self, id: UserId) -> Result<User> {
        let result: entities::User = sqlx::query_as("SELECT * FROM Users WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(User {
            id: UserId(result.id),
            is_admin: result.is_admin,
        })
    }

    async fn all(&self) -> Result<Vec<User>> {
        let users: Vec<entities::User> = sqlx::query_as("SELECT * FROM Users")
            .fetch_all(&*self.0)
            .await?;
        Ok(users
            .into_iter()
            .map(|u| User {
                id: UserId(u.id),
                is_admin: u.is_admin,
            })
            .collect())
    }

    async fn admins(&self) -> Result<Vec<User>> {
        let users: Vec<entities::User> =
            sqlx::query_as("SELECT * FROM Users WHERE is_admin = TRUE")
                .fetch_all(&*self.0)
                .await?;
        Ok(users
            .into_iter()
            .map(|u| User {
                id: UserId(u.id),
                is_admin: u.is_admin,
            })
            .collect())
    }

    async fn non_admins(&self) -> Result<Vec<User>> {
        let users: Vec<entities::User> =
            sqlx::query_as("SELECT * FROM Users WHERE is_admin = FALSE")
                .fetch_all(&*self.0)
                .await?;
        Ok(users
            .into_iter()
            .map(|u| User {
                id: UserId(u.id),
                is_admin: u.is_admin,
            })
            .collect())
    }

    // - related
    async fn fiefs(&self, id: UserId) -> Result<Vec<FiefId>> {
        let fiefs: Vec<(i64,)> = sqlx::query_as("SELECT fief_id FROM Members WHERE user_id = $1")
            .bind(id.0)
            .fetch_all(&*self.0)
            .await?;
        Ok(fiefs.into_iter().map(|f| FiefId(f.0)).collect())
    }

    async fn is_member_of(&self, id: UserId, fief_id: FiefId) -> Result<bool> {
        let (result,): (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM Members WHERE user_id = $1 AND fief_id = $2)",
        )
        .bind(id.0)
        .bind(fief_id.0)
        .fetch_one(&*self.0)
        .await?;
        Ok(result)
    }

    async fn permissions_in(&self, id: UserId, fief_id: FiefId) -> Result<Permissions> {
        let (p,): (i64,) =
            sqlx::query_as("SELECT permissions FROM Members WHERE user_id = $1 AND fief_id = $2")
                .bind(id.0)
                .bind(fief_id.0)
                .fetch_one(&*self.0)
                .await?;
        Ok(Permissions::from_bits(p)
            .ok_or(anyhow::anyhow!("failed to parse permissions from database"))?)
    }

    // [U] Update
    // - self or fields
    async fn set_admin(&self, id: UserId, is_admin: bool) -> Result<()> {
        sqlx::query("UPDATE Users SET is_admin = $1 WHERE id = $2")
            .bind(is_admin)
            .bind(id.0)
            .execute(&*self.0)
            .await?;

        Ok(())
    }

    // - related
    async fn set_permissions_in(&self, id: UserId, fief_id: FiefId, p: Permissions) -> Result<()> {
        sqlx::query("UPDATE Members SET permissions = $1 WHERE user_id = $2 AND fief_id = $3")
            .bind(p.bits())
            .bind(id.0)
            .bind(fief_id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    // [D] Delete
    async fn leave(&self, id: UserId, fief_id: FiefId) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Members WHERE user_id = $1 AND fief_id = $2")
            .bind(id.0)
            .bind(fief_id.0)
            .execute(&*self.0)
            .await?;
        Ok(result.rows_affected() == 1)
    }

    async fn remove_by_id(&self, id: UserId) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Users WHERE id = $1")
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(result.rows_affected() == 1)
    }
}
