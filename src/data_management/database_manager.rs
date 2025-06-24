
use sled::{Db};

use crate::{data_management::user_data::UserData};


pub struct DbManager {
    db: Db
}

const CFG: bincode::config::Configuration = bincode::config::standard();

impl DbManager {
    pub fn new(path: &str) -> DbManager {
        let db1;
        match sled::open(path) {
            Ok(database) => {
                db1 = database;
            },
            Err(err) => {
                println!("{}", err);
                std::process::exit(2);
            },
        }
        DbManager { db: db1 }
    }

    pub fn save_user_data(&self, data: &UserData) {
        let binary_data;
        match bincode::encode_to_vec(&data, CFG) {
            Ok(val) => {
                binary_data = val;
            },
            Err(_) => {
                println!("Failed to save data from user {}", data.id);
                return
            },
        }
        let _ = self.db.insert(data.id.to_be_bytes(), binary_data);
    }

    pub fn load_user_data(&self, id: u64) -> Option<UserData> {
        let data_result = self.db.get(id.to_be_bytes());
        let data_ivec;
        match data_result {
            Ok(value) => {
                data_ivec = value;
            },
            Err(err) => {
                println!("Data not loaded: {}", err);
                return None;
            },
        }
        let data_vec;
        match data_ivec {
            Some(var) => {
                data_vec = var.to_vec();
            },
            None => {
                println!("Ivec was empty");
                return None
            },
        }

        let decoded_data: Result<(UserData, usize), bincode::error::DecodeError> = bincode::decode_from_slice(&data_vec.to_vec(),  CFG);
        match decoded_data {
            Ok(data) => Some(data.0.clone()),
            Err(err) => {
                println!("Error decoding user data: {}", err);
                None
            },
        }
    }

}

#[allow(dead_code)]
pub fn test_db() {
    let db = DbManager::new("my_db.db");
    let data = UserData::new(0);
    db.save_user_data(&data);
    match db.load_user_data(0) {
        Some(val) => {
            assert_eq!(data, val);
        },
        None => {
            println!("Something funky is goin' on");
            std::process::exit(3)
        },
    }
}
