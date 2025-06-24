use serenity::{all::{CommandInteraction, Context, GuildId, Interaction}, builder};

use crate::{command_utils, data_management::user_data::UserData, get_user_cache};



#[allow(dead_code)]
pub async fn load_command(context: &Context) {
    let _ = GuildId::new(1306325382697582652).create_command(context, 
        builder::CreateCommand::new("additem")
        .description("Adds an item to your inventory")
        .add_option(builder::CreateCommandOption::new(serenity::all::CommandOptionType::String, "itemid", "Put the item id of the item you wish to receive").required(true))
        .add_option(builder::CreateCommandOption::new(serenity::all::CommandOptionType::String, "itemcount", "Put how many items you want").required(true))
    ).await;
    println!("Loaded add item command")
}

pub async fn run_command(cmd: &CommandInteraction, context: Context, _interaction: &Interaction) {
    
    let mut cache = get_user_cache().await;
    let mut user_data = match cache.get_user_data(cmd.user.id.get()) {
        Some(data) => data,
        None => {
            println!("User Data Not Found\nCreating New User Data");
            let user_data1 = UserData::new(cmd.user.id.get());
            let id = user_data1.id;
            cache.save_user_data(user_data1);
            cache.get_user_data(id).unwrap()
        },
    };

    let item_id: Option<u64> = command_utils::extract_string_option(cmd.data.options.get(0))
        .and_then(|s| s.parse::<u64>().ok());

    let count: Option<u64> = command_utils::extract_string_option(cmd.data.options.get(1))
        .and_then(|s| s.parse::<u64>().ok());

    let final_content;

    if item_id.is_some() && count.is_some(){
        user_data.add_item(item_id.unwrap(), count.unwrap());
        cache.save_user_data(user_data);
        final_content = "Item Added"
    } else {
        final_content = "Bad Args"
    }

    let _ = cmd.create_response(
        context,
        builder::CreateInteractionResponse::Message(
            builder::CreateInteractionResponseMessage::new()
                .content(final_content)
                .ephemeral(true),
        ),
    )
    .await;
    println!("Recieved command additem from user {}. args were {:?}, {:?}", cmd.user.id, item_id, count)
}

