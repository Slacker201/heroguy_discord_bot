use bincode::{Decode, Encode};

use crate::{gen_random, heroguy_gatcha::item_rarities::ItemRarity};


#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct GatchaChance {
    chances: Vec<u32>,
    draw_count: Vec<u32>,
}

#[allow(dead_code)]
impl GatchaChance {
    pub fn new() -> GatchaChance {
        GatchaChance { chances: Vec::from([2147483647, 536870911, 322122547, 214748364, 85899345, 32212254, 42949673, 10737418, 429496]), draw_count: Vec::from([5, 3, 1, 1, 1, 1, 1, 1, 1]) }
    }
    pub fn get_chance(&self, index: usize) -> Option<u32> {
        self.chances.get(index).copied()
    }
    pub async fn draw_item_rarities(&mut self) -> Vec<(u32, ItemRarity)> {
        let mut item_count = Vec::new();
        for rarity in ItemRarity::get_all() {
            let chance = &self.draw_for_rarity(&rarity, self.draw_count[rarity.clone() as usize]).await;
            item_count.push((*chance, rarity));
        }
        item_count
    }


    pub async fn draw_for_rarity(&mut self, rarity: &ItemRarity, draw_amount: u32) -> u32 {
        let chance = &self.get_rarity_chance(rarity);
        let mut count = 0;
        for _ in 0..draw_amount {
            if gen_random().await < *chance {
                count += 1;
            }
        }
        count
    }


    pub fn get_rarity_chance(&self, rarity: &ItemRarity) -> u32 {
        *self.chances.get(rarity.clone() as usize).unwrap_or(&0)
    }


}