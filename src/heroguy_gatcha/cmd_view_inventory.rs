use serenity::{all::{CommandInteraction, Context, GuildId, Interaction}, builder};

use crate::{data_management::user_data::{ UserData}, get_user_cache};



#[allow(dead_code)]
pub async fn load_command(context: &Context) {
    let _ = GuildId::new(1306325382697582652).create_command(context, 
        builder::CreateCommand::new("viewinventory")
        .description("returns your inventory")
    ).await;
    println!("Loaded view inventory command")
}

pub async fn run_command(cmd: &CommandInteraction, context: Context, _interaction: &Interaction) {
    let mut cache = get_user_cache().await;
    let user_data;
    match cache.get_user_data(cmd.user.id.get()) {
        Some(data) => {
            user_data = data;
        },
        None => {
            println!("User Data Not Found\nCreating New User Data");
            let user_data1 = UserData::new(cmd.user.id.get());
            let id = user_data1.id;
            cache.save_user_data(user_data1);
            user_data = cache.get_user_data(id).unwrap();
        },
    }

    let heading = "Inventory";
    let separator = "---------";
    let body = user_data.inventory_to_string();
    let content = format!("{}\n{}\n{}", heading, separator, body);

    let _ = cmd.create_response(
        context,
        builder::CreateInteractionResponse::Message(
            builder::CreateInteractionResponseMessage::new()
                .content(content)
                .ephemeral(true),
        ),
    )
    .await;

    
}
