use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::gateway::Ready;
use tracing::{error, info};

use crate::commands;

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => "bacot".to_string(),
            };

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
            }).await {
                error!("Failed to respond to command: {e}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Register commands
        for guild in ready.guilds {
            if !guild.unavailable {
                match guild.id.set_application_commands(&ctx.http, |commands| {
                    commands
                        .create_application_command(|command| commands::ping::register(command))
                }).await {
                    Ok(commands) => info!("Registered commands for guild {:#?}: {:#?}", guild, commands),
                    Err(e) => error!("Failed to register commands: {e}")
                }
            }
        }
    }
}
