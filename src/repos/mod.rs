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

mod repos;
pub mod sqlx;
pub use repos::Repositories;
