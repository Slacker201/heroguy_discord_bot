use std::{fs::File, io::{Error, Read}};

use serenity::{
    all::Interaction, async_trait, model::{gateway::Ready}, prelude::*
};

use crate::{commands::*, heroguy_gatcha::{cmd_add_item, cmd_gatcha_draw, cmd_remove_item, cmd_view_inventory}};

struct Handler;


/// The event handler for the bot

#[async_trait]
impl EventHandler for Handler {
    /// load commands in this function. Only load them when options or name changes, otherwise it just slows down initlialization
    async fn ready(&self, context: Context, ready: Ready) {
        println!("Creating Commands");
        // Init commands

        let _ = context;
        // Log connection
        println!("{} is connected!", ready.user.name);
    }


    /// Listen for commands here
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Interaction::Command(command) = &interaction {
            println!("Recieved command {} from user {}", command.data.name, command.user.id);
            match command.data.name.as_str() {
                "ping" => {
                    cmd_ping::run_command(command, context, &interaction).await;
                }
                "eping" => {
                    cmd_eping::run_command(command, context, &interaction).await;
                }
                "echo" => {
                    cmd_echo::run_command(command, context, &interaction).await;
                }
                "viewinventory" => {
                    cmd_view_inventory::run_command(command, context, &interaction).await;
                }
                "additem" => {
                    cmd_add_item::run_command(command, context, &interaction).await;
                }
                "removeitem" => {
                    cmd_remove_item::run_command(command, context, &interaction).await;
                }
                "draw" => {
                    cmd_gatcha_draw::run_command(command, context, &interaction).await;
                }
                _ => {}
            }
        }
    }
}

/// Create the client
pub async fn init_client(token: String, intents: GatewayIntents) {


    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
/// load_token loads the bot token from the supplied file path
pub fn load_token(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);
    match String::from_utf8(buf) {
        Ok(var) => Ok(var),
        Err(error) => {
            Err(Error::new(std::io::ErrorKind::InvalidInput, error))
        },
    }
}