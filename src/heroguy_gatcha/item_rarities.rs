use bincode::{Decode, Encode};

use crate::heroguy_gatcha::item_rarities::ItemRarity::*;



#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum ItemRarity {
    Common = 0, // 50%
    Uncommon = 1, // 25%
    Rare = 2, // 7.5%
    Epic = 3, // 5%
    Legendary = 4, // 2%
    Mythic = 5, // 0.75%
    Ancient = 6,    // Wildcard 1%
    Artifact = 7,   // Wildcard 0.25%
    Trophy = 8, // Wildcard 0.01%
}


impl ItemRarity {
    pub fn get_all() -> Vec<ItemRarity> {
        Vec::from([Common, Uncommon, Rare, Epic, Legendary, Mythic, Ancient, Artifact, Trophy])
    }
    pub fn get_enum_count() -> usize {
        9
    }
}