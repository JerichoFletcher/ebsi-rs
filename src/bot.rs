use std::{sync::Mutex, collections::BTreeMap};

use serenity::async_trait;
use serenity::model::prelude::command::Command;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::gateway::Ready;
use tracing::{error, info};

use crate::{model::command::CommandTrait, commands::{
    ping::PingCommand,
    dice::DiceCommand,
    help::HelpCommand
}};

pub struct Bot {
    start_timestamp: Mutex<Option<Timestamp>>,
    user_avatar_url: Mutex<Option<String>>,
    pub handler_map: Mutex<BTreeMap<String, Box<dyn CommandTrait>>>,
    pub cmdinfo_map: Mutex<BTreeMap<String, Command>>,
}

impl Bot {
    pub fn new() -> Self {
        Self {
            start_timestamp: Mutex::new(None),
            user_avatar_url: Mutex::new(None),
            handler_map: Mutex::new(BTreeMap::new()),
            cmdinfo_map: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn start_timestamp(&self) -> Option<Timestamp> {
        *self.start_timestamp.lock().unwrap()
    }

    pub fn user_avatar_url(&self) -> Option<String> {
        self.user_avatar_url.lock().unwrap().as_ref().cloned()
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    match self.handler_map.lock().unwrap().get(command.data.name.as_str()) {
                        Some(cmd) => {
                            cmd.run(self, &command.data.options, response);
                            response
                        },
                        None => response
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

        // Set local variables
        *self.start_timestamp.lock().unwrap() = Some(Timestamp::now());
        if let Some(url) = ready.user.avatar_url() {
            *self.user_avatar_url.lock().unwrap() = Some(url);
        }

        // Register commands
        match Command::set_global_application_commands(&ctx.http, |commands| {
            let mut map = self.handler_map.lock().unwrap();
            let cmd_list: [Box<dyn CommandTrait>; 3] = [
                Box::new(HelpCommand),
                Box::new(PingCommand),
                Box::new(DiceCommand),
            ];

            for cmd in cmd_list {
                let map = &mut *map;
                commands.create_application_command(move |command| {
                    cmd.reg(command);
                    map.insert(cmd.name().into(), cmd);
                    command
                });
            }

            commands
        }).await {
            Ok(commands) => {
                let mut map = self.cmdinfo_map.lock().unwrap();

                for cmd in commands {
                    info!("Registered command: {}", cmd.name);
                    map.insert(cmd.name.clone(), cmd);
                }
            },
            Err(e) => error!("Failed to register commands: {e}")
        }
    }
}
