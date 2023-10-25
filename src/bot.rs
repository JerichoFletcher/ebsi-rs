use serenity::async_trait;
use serenity::model::prelude::command::Command;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::gateway::Ready;
use tracing::{error, info};

use crate::commands;

static mut START_TIMESTAMP: Option<Timestamp> = Option::None;
static mut SELF_USER: Option<CurrentUser> = Option::None;

pub struct Bot;

impl Bot {
    pub fn start_timestamp() -> Option<Timestamp> {
        unsafe {
            START_TIMESTAMP
        }
    }

    pub fn self_user() -> &'static Option<CurrentUser> {
        unsafe {
            &SELF_USER
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    match command.data.name.as_str() {
                        "ping" => {
                            commands::ping::run(&command.data.options, response);
                            response
                        },
                        _ => response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("Oh no"))
                    }
            }).await {
                error!("Failed to respond to command: {e}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Set static variables
        unsafe {
            START_TIMESTAMP = Option::Some(Timestamp::now());
            SELF_USER = Option::Some(ready.user);
        }

        // Register commands
        match Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
        }).await {
            Ok(commands) => {
                let commands: Vec<&String> = commands.iter().map(|command| &command.name).collect();
                info!("Registered commands: {:#?}", commands);
            },
            Err(e) => error!("Failed to register commands: {e}")
        }
    }
}
