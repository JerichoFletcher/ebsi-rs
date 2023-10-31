use std::time::Duration;

use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::Timestamp;
use serenity::model::prelude::{application_command::CommandDataOption, InteractionResponseType};

use crate::bot::Bot;
use crate::model::command::CommandTrait;
use crate::utils::{embed, time_format};

#[derive(Clone, Copy)]
pub struct PingCommand;

impl CommandTrait for PingCommand {
    fn name(&self) -> &str {
        "ping"
    }
    
    fn run(&self, bot: &Bot, _options: &[CommandDataOption], response: &mut CreateInteractionResponse) {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message
                .embed(|e| {
                    let time = bot.start_timestamp().unwrap();
                    
                    let start = time.unix_timestamp();
                    let now = Timestamp::now().unix_timestamp();
                    let dif = Duration::from_secs((now - start).unsigned_abs());
                    
                    embed::template_ok(e, bot, "Apaan sih?");
                    e
                    .description("Brisik tau ga")
                    .field("Online sejak", time_format::timestamp_to_string(&time), true)
                    .field("\u{200E}", "\u{200E}", true)
                    .field("Online selama", time_format::duration_to_string(&dif), true)
                })
            );
    }
    
    fn reg<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command
            .name(self.name())
            .description("Sapa EBSI dengan ramah!")
    }
}
