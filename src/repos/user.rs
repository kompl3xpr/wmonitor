use crate::domains::FiefId;
use anyhow::Result;
use async_trait::async_trait;

pub(super) mod domains {
    use bitflags::bitflags;
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
    pub struct UserId(pub i64);

    impl From<i64> for UserId {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct User {
        pub id: UserId,
        pub is_admin: bool,
    }

    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Permissions: i64 {
            /// 领地编辑权
            const FIEF_EDIT           = 1 << 0;
            /// 领地删除权
            const FIEF_DELETE         = 1 << 1;
            /// 区块添加权
            const CHUNK_ADD           = 1 << 2;
            /// 区块编辑权
            const CHUNK_EDIT          = 1 << 3;
            /// 区块删除权
            const CHUNK_DELETE        = 1 << 4;
            /// 添加成员权
            const MEMBER_INVITE       = 1 << 5;
            /// 编辑成员权限权
            const MEMBER_EDIT_PERMS   = 1 << 6;
            /// 踢出成员权
            const MEMBER_KICK         = 1 << 7;

            const NONE = 0;
            const CHUNK_ALL = Self::CHUNK_ADD.bits() | Self::CHUNK_EDIT.bits() | Self::CHUNK_DELETE.bits();
            const FIEF_ALL = Self::FIEF_EDIT.bits() | Self::FIEF_DELETE.bits();
            const MEMBER_ALL = Self::MEMBER_INVITE.bits() | Self::MEMBER_EDIT_PERMS.bits() | Self::MEMBER_KICK.bits();
            const ALL = Self::CHUNK_ALL.bits() | Self::FIEF_ALL.bits() | Self::MEMBER_ALL.bits();
        }
    }
}
use domains::*;

#[async_trait]
pub trait UserRepo: Sync + Send {
    fn clone(&self) -> Box<dyn UserRepo>;

    // [C]reate
    async fn create(&self, id: UserId, is_admin: bool) -> Result<Option<UserId>>;
    async fn join(&self, id: UserId, fief_id: FiefId, p: Option<Permissions>) -> Result<bool>;

    // [R]ead
    // - self or fields
    async fn user_by_id(&self, id: UserId) -> Result<User>;
    async fn all(&self) -> Result<Vec<User>>;
    async fn admins(&self) -> Result<Vec<User>>;
    async fn non_admins(&self) -> Result<Vec<User>>;
    // - related
    async fn fiefs(&self, id: UserId) -> Result<Vec<FiefId>>;
    async fn is_member_of(&self, id: UserId, fief_id: FiefId) -> Result<bool>;
    async fn permissions_in(&self, id: UserId, fief_id: FiefId) -> Result<Permissions>;

    // [U]pdate
    // - self or fields
    async fn set_admin(&self, id: UserId, is_admin: bool) -> Result<()>;
    // - related
    async fn set_permissions_in(&self, id: UserId, fief_id: FiefId, p: Permissions) -> Result<()>;

    // [D]elete
    async fn remove_by_id(&self, id: UserId) -> Result<bool>;
    async fn leave(&self, id: UserId, fief_id: FiefId) -> Result<bool>;
}
