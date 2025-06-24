
use serenity::{all::{CommandInteraction, Context, GuildId, Interaction}, builder};

use crate::{get_item_name_from_id, get_user_cache};



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
    let content;
    let count;
    match cache.get_user_data(cmd.user.id.get()) {
        Some(mut user_data) => {
            user_data.add_item(1, 1);
            content = get_item_name_from_id(1);
            count = 1;
        },
        None => {
            content = "Invalid Item".to_string();
            count = 0
        },
    }
    let err = cmd.create_response(context, builder::CreateInteractionResponse::Message(builder::CreateInteractionResponseMessage::new()
            .content(format!("Drew {} {}", count, content)).ephemeral(true))).await;
    if let Err(error) = err {
        println!("Error sending message: {:?}", error);
    }
}