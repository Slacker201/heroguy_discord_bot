
use serenity::{all::{CommandInteraction, Context, GuildId, Interaction}, builder};

use crate::{data_management::item_manager, get_user_cache};



#[allow(dead_code)]
pub async fn load_command(context: &Context) {
    let _ = GuildId::new(1306325382697582652).create_command(context, 
        builder::CreateCommand::new("draw")
        .description("Perform a draw to receive items based on rarity")
    ).await;
    println!("Loaded draw command")
}

pub async fn run_command(cmd: &CommandInteraction, context: Context, _interaction: &Interaction) {
    println!("Recieved draw request from user {}", cmd.user.id);
    let mut cache = get_user_cache().await;
    let drawn_item_name;
    let count;
    let mut chosen_item_ids = Vec::new();
    match cache.get_user_data(cmd.user.id.get()) {
        Some(mut user_data) => {
            let rarity_entries = user_data.gatcha_chance.draw_item_rarities().await;
            for rarity_entry in rarity_entries {
                for _ in 0..rarity_entry.0 {
                    chosen_item_ids.push(item_manager::pick_random_item_by_rarity(rarity_entry.1.clone()));
                }
            }
            drawn_item_name = "idk".to_string();
            count = 1;
        },
        None => {
            drawn_item_name = "Invalid Item".to_string();
            count = 0
        },
    }
    let err = cmd.create_response(context, builder::CreateInteractionResponse::Message(builder::CreateInteractionResponseMessage::new()
            .content(format!("Drew {} {}", count, drawn_item_name)).ephemeral(true))).await;
    if let Err(error) = err {
        println!("Error sending message: {:?}", error);
    }
}