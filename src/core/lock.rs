use dashmap::DashMap;
use std::sync::{LazyLock, Arc};
use tokio::sync::Mutex;

use crate::domains::FiefId;

static FIEF_LOCKS: LazyLock<DashMap<FiefId, Arc<Mutex<()>>>> = LazyLock::new(|| DashMap::new());

pub struct FiefLock(
    pub Arc<Mutex<()>>,
    // FiefId,
);

impl Drop for FiefLock {
    fn drop(&mut self) {
        // FIEF_LOCKS.remove(&self.1);
    }
}

pub fn get_fief_lock(fief: FiefId) -> FiefLock {
    if !FIEF_LOCKS.contains_key(&fief) {
        FIEF_LOCKS.insert(fief, Arc::new(Mutex::new(())));
    }

    FiefLock(
        FIEF_LOCKS.get(&fief).unwrap().clone(),
        // fief, 
    )
}

macro_rules! lock_fief {
    ($fief_id:expr) => {
        let _lock_of_fief = $crate::core::lock::get_fief_lock($fief_id);
        let _lock_guard_of_fief = _lock_of_fief.0.lock().await;
    };
}

pub(crate) use lock_fief;