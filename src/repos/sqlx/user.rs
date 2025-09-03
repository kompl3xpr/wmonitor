use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use crate::repos::traits::UserRepo;
use async_trait::async_trait;
use crate::domains::{User, UserId, Permissions, FiefId};
use anyhow::Result;

pub struct SqlxUserRepo(Arc<SqlitePool>);

impl SqlxUserRepo {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self(pool)
    }
}

#[allow(unused)]
#[async_trait]
impl UserRepo for SqlxUserRepo {
    // [C]reate
    async fn create(&self, id: UserId, is_admin: bool) -> Result<()> {
        todo!()
    }
    
    // [R]ead
    // - self or fields
    async fn user_by_id(&self, id: UserId) -> Result<User> {
        todo!()
    }

    async fn all(&self) -> Result<Vec<User>> {
        todo!()
    }
    async fn admins(&self) -> Result<Vec<User>> {
        todo!()
    }
    
    async fn non_admins(&self) -> Result<Vec<User>> {
        todo!()
    }
    
    // - related
    async fn fiefs(&self, id: UserId) -> Result<Vec<FiefId>> {
        todo!()
    }
    
    async fn is_member_of(&self, id: UserId, fief_id: FiefId) -> Result<bool> {
        todo!()
    }
    
    async fn permissions_in(&self, id: UserId, fief_id: FiefId) -> Result<Permissions> {
        todo!()
    }
    
    
    // [U]pdate
    // - self or fields
    async fn set_admin(&self, id: UserId) -> Result<bool> {
        todo!()
    }
    
    // - related
    async fn set_permissions_in(&self, id: UserId, fief_id: FiefId, p: Permissions) -> Result<()> {
        todo!()
    }
    
    async fn join(&self, id: UserId, fief_id: FiefId, p: Option<Permissions>) -> Result<()> {
        todo!()
    }
    
    async fn leave(&self, id: UserId, fief_id: FiefId) -> Result<bool> {
        todo!()
    }
    
    
    // [D]elete
    async fn remove_by_id(&self, id: UserId) -> Result<()> {
        todo!()
    }
    
}