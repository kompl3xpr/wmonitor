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

pub mod sqlx;
mod repos;
pub use repos::Repositories;