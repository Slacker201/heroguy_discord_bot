use std::collections::HashMap;

use bincode::{Decode, Encode};

use crate::{get_item_name_from_id, heroguy_gatcha::{gatcha_chance::GatchaChance, item_rarities::ItemRarity}, item_id_to_rarity};




#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct UserData {
    pub id: u64,
    xp: u128,
    inventory: HashMap<u64, Item>,
    gatcha_chance: GatchaChance
    
}
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub count: u128,
    pub rarity: ItemRarity,
    pub metadata: Vec<u8>
}
impl UserData {
    pub fn new(id: u64) -> UserData {
        println!("Creating new user data with id: {}", id);
        UserData { id: id, xp: 0, inventory: HashMap::new(), gatcha_chance: GatchaChance::new()}
    }
    
    pub fn inventory_to_string(self) -> String {
        let mut r = String::new();

        for item in self.inventory {
            let name = get_item_name_from_id(item.1.id);
            let rarity = item.1.rarity;
            r += &format!("{} {:?} {}\n", item.1.count, rarity, name)
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
                rarity: item_id_to_rarity(id),
            });
    }

    pub fn get_item(&mut self, id: u64) -> Option<&mut Item> {
        self.inventory.get_mut(&id)
    }

}