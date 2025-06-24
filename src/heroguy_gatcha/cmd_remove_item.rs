

use serenity::{all::{CommandInteraction, Context, GuildId, Interaction}, builder};

use crate::{command_utils, data_management::user_data::UserData, get_user_cache};



#[allow(dead_code)]
pub async fn load_command(context: &Context) {
    let _ = GuildId::new(1306325382697582652).create_command(context, 
        builder::CreateCommand::new("removeitem")
        .description("Removes an item from your inventory")
        .add_option(builder::CreateCommandOption::new(serenity::all::CommandOptionType::String, "itemid", "Paste the item id here").required(true))
        .add_option(builder::CreateCommandOption::new(serenity::all::CommandOptionType::String, "itemcount", "How many items do you want to remove").required(true))
    ).await;
    println!("Loaded removeitem command")
}

pub async fn run_command(cmd: &CommandInteraction, context: Context, _interaction: &Interaction) {
    let mut cache = get_user_cache().await;
    let user_id = cmd.user.id.get();
    
    let mut user_data = cache.get_user_data(user_id).unwrap_or_else(|| {
        println!("User Data Not Found\nCreating New User Data");
        let new_data = UserData::new(user_id);
        cache.save_user_data(new_data);
        cache.get_user_data(user_id).unwrap()
    });

    let item_id = match command_utils::extract_string_option(cmd.data.options.get(0))
        .and_then(|s| s.parse::<u64>().ok()) {
            Some(id) => id,
            None => {
                println!("Invalid or missing item ID");
                return;
            }
        };

    let count = match command_utils::extract_string_option(cmd.data.options.get(1))
        .and_then(|s| s.parse::<u128>().ok()) {
            Some(c) => c,
            None => {
                println!("Invalid or missing count");
                return;
            }
        };

    match user_data.get_item(item_id) {
        Some(item) => {
            println!("Removing item id: {}, count: {}", item_id, count);
            item.count = item.count.saturating_sub(count);
            cache.save_user_data(user_data);
        },
        None => println!("Item not found"),
    }

    let _ = cmd.create_response(context, builder::CreateInteractionResponse::Message(builder::CreateInteractionResponseMessage::new().content("Recieved Command").ephemeral(true))).await;
}
