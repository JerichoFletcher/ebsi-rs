use serenity::async_trait;
use serenity::model::prelude::command::Command;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::gateway::Ready;
use tracing::{error, info};

use crate::{commands::ping::PingCommand, model::command::CommandTrait};

static mut START_TIMESTAMP: Option<Timestamp> = None;
static mut USER_AVATAR_URL: Option<String> = None;

pub struct Bot;

impl Bot {
    pub fn start_timestamp() -> Timestamp {
        unsafe {
            START_TIMESTAMP.unwrap_or(Timestamp::now())
        }
    }

    pub fn user_avatar_url() -> &'static str {
        unsafe {
            USER_AVATAR_URL.as_ref().unwrap().as_str()
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
                        "ping" => { PingCommand::run(&command.data.options, response); response },
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
            START_TIMESTAMP = Some(Timestamp::now());
            match ready.user.avatar_url() {
                Some(url) => {
                    USER_AVATAR_URL = Some(url);
                },
                None => ()
            }
        }

        // Register commands
        match Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| PingCommand::reg(command))
        }).await {
            Ok(commands) => {
                let commands: Vec<&String> = commands.iter().map(|command| &command.name).collect();
                info!("Registered commands: {:#?}", commands);
            },
            Err(e) => error!("Failed to register commands: {e}")
        }
    }
}
