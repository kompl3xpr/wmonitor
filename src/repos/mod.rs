mod chunk;
mod event;
mod fief;
mod user;

pub mod domains {
    pub use super::user::domains::*;
    pub use super::fief::domains::*;
    pub use super::chunk::domains::*;
    pub use super::event::domains::*;
}

pub mod traits {
    pub use super::chunk::ChunkRepo;
    pub use super::event::EventRepo;
    pub use super::fief::FiefRepo;
    pub use super::user::UserRepo;
}

pub mod sqlx {
    pub use super::chunk::SqlxChunkRepo;
    pub use super::event::SqlxEventRepo;
    pub use super::fief::SqlxFiefRepo;
    pub use super::user::SqlxUserRepo;
}
