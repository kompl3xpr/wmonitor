mod chunk;
mod event;
mod fief;
mod user;

pub mod domains {
    pub use super::chunk::Chunk;
    pub use super::event::Event;
    pub use super::fief::Fief;
    pub use super::user::User;
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
