
use serenity::{all::{CommandInteraction, Context, GuildId, Interaction}, builder};



#[allow(dead_code)]
pub async fn load_command(context: &Context) {
    let _ = GuildId::new(1306325382697582652).create_command(
        context,
        builder::CreateCommand::new("echo")
            .description("Says what you did")
            .add_option(
                builder::CreateCommandOption::new(
                    serenity::all::CommandOptionType::String,
                    "text",
                    "Put what you want echoed",
                )
                .required(true),
            ),
    ).await;
    println!("Loaded echo command")
}
pub async fn run_command(cmd: &CommandInteraction, context: Context, _interaction: &Interaction) {
    let v1 = cmd.data.options.get(0);
    let content;
    let echo;
    match v1 {
        Some(v2) => {
            match v2.value.as_str() {
                Some(value) => {
                    content = "Successfully loaded content";
                    echo = value
                },
                None => {
                    content = "Somehow your argument didnt turn into a string";
                    echo = "Value didn't turn into a string"
                },
            }
        },
        None => {
            println!("Apparently there were no args in echo command");
            content = "Invalid Arguments";
            echo = "Invalid Arguments";
        },
    }
    



    let err2 = cmd.create_response(&context, builder::CreateInteractionResponse::Message(builder::CreateInteractionResponseMessage::new().content(content).ephemeral(true))).await;
    let err = cmd.channel_id.say(&context, echo).await;
    
    if let Err(error) = err {
        println!("Error sending message: {:?}", error);
    }
    if let Err(error) = err2 {
        println!("Error2 sending message: {:?}", error);
    }

    println!("Recieved echo request from user {}. Text is {}", cmd.user.id, echo);
}