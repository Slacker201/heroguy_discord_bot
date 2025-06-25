use std::{collections::HashMap};

use once_cell::sync::Lazy;

use crate::{heroguy_gatcha::item_rarities::ItemRarity, pick_random};

pub static ITEM_LOOKUP_TABLE: Lazy<HashMap<u64, (&'static str, ItemRarity)>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, ("Ink", ItemRarity::Common));
    m.insert(2, ("T1 Quill", ItemRarity::Common));
    m.insert(3, ("T1 Paper", ItemRarity::Common));
    m
});

pub static RARITY_TO_ITEM_TABLE: Lazy<HashMap<ItemRarity, Vec<u64>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(ItemRarity::Common, Vec::from([1, 2, 3]));
    m
});
pub fn get_item_name_from_id(id: u64) -> String{
    match ITEM_LOOKUP_TABLE.get(&id) {
        Some(name) => format!("{} {}", name.1, name.0),
        None => id.to_string(),
    }
}

pub fn item_id_to_rarity(_id: u64) -> ItemRarity {
    ItemRarity::Common
}

pub async fn pick_random_item_by_rarity(rarity: ItemRarity) -> u64 {
    let ids = RARITY_TO_ITEM_TABLE.get(&rarity);
    match ids {
        Some(ids2) => {
            return *pick_random(ids2).await.unwrap_or(&0);
        },
        None => 0,
    }
}