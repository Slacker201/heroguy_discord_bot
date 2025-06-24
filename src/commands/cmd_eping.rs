
use serenity::{all::{CommandInteraction, Context, GuildId, Interaction}, builder};



#[allow(dead_code)]
pub async fn load_command(context: &Context) {
    let _ = GuildId::new(1306325382697582652).create_command(context, 
        builder::CreateCommand::new("ping")
        .description("The standard /ping command, except only you can see the response")
    ).await;
    println!("Loaded eping command")
}

pub async fn run_command(cmd: &CommandInteraction, context: Context, _interaction: &Interaction) {
    let err = cmd.create_response(context, builder::CreateInteractionResponse::Message(builder::CreateInteractionResponseMessage::new()
            .content("Pong!")
            .ephemeral(true))).await;
    if let Err(error) = err {
        println!("Error sending message: {:?}", error);
    }
}