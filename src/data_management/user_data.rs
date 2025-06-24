use std::collections::HashMap;

use bincode::{Decode, Encode};

use crate::ITEM_LOOKUP_TABLE;




#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct UserData {
    pub id: u64,
    xp: u128,
    inventory: HashMap<u64, Item>
}
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub count: u128,
    pub metadata: Vec<u8>
}
impl UserData {
    pub fn new(id: u64) -> UserData {
        println!("Creating new user data with id: {}", id);
        UserData { id: id, xp: 0, inventory: HashMap::new() }
    }
    
    pub fn inventory_to_string(self) -> String {
        let mut r = String::new();

        for item in self.inventory {
            let name: &str;
            match ITEM_LOOKUP_TABLE.get(&item.0) {
                Some(name1) => {
                    name = name1;
                },
                None => {
                    name = "Invalid Item"
                },
            }
            r += &format!("{} {}\n", item.1.count, name)
        }
        r.trim().to_owned()
    }

    pub fn add_item(&mut self, id: u64, count: u64) {
        self.inventory
            .entry(id)
            .and_modify(|item| item.count += count as u128)
            .or_insert(Item {
                id,
                count: count as u128,
                metadata: Vec::new(),
            });
    }

    pub fn get_item(&mut self, id: u64) -> Option<&mut Item> {
        self.inventory.get_mut(&id)
    }

}