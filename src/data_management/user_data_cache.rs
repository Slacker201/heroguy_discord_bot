use std::num::NonZero;

use lru::LruCache;

use crate::data_management::{database_manager::DbManager, user_data::UserData};




const MAX_CACHE: usize = 10_000;

pub struct UserCache {
    cache: LruCache<u64, UserData>,
    db_manager: DbManager
}


impl UserCache {
    pub fn new(path: &str) -> UserCache {
        UserCache {
            cache: LruCache::new(NonZero::new(MAX_CACHE).unwrap()),
            db_manager: DbManager::new(path),
        }
    }

    // Returns owned UserData (cloned from cache or loaded from DB)
    pub fn get_user_data(&mut self, id: u64) -> Option<UserData> {
        if let Some(data) = self.cache.get(&id) {
            Some(data.clone())
        } else {
            let d = self.db_manager.load_user_data(id);
            match &d {
                Some(var) => {
                    self.cache.put(var.id, var.clone());
                },
                None => {
                    return None
                },
            }
            d
        }
    }

    pub fn save_user_data(&mut self, data: UserData) { 
        self.db_manager.save_user_data(&data);
        self.cache.put(data.id, data);
       
    }
}
