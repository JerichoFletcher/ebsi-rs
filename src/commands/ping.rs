use std::time::Duration;

use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::Timestamp;
use serenity::model::prelude::{application_command::CommandDataOption, InteractionResponseType};

use crate::bot::Bot;
use crate::utils::{embed, time_format};

pub fn run(_options: &[CommandDataOption], response: &mut CreateInteractionResponse) {
    response
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|message| message
            .embed(|e| {
                let start = Bot::start_timestamp().unwrap().unix_timestamp();
                let now = Timestamp::now().unix_timestamp();
                let dif = Duration::from_secs((now - start).unsigned_abs());
                    
                embed::template_ok(e, "Apaan sih?");
                e
                    .description("Brisik tau ga")
                    .field("Online sejak", time_format::timestamp_to_string(&Bot::start_timestamp().unwrap()), true)
                    .field("\u{200E}", "\u{200E}", true)
                    .field("Online selama", time_format::duration_to_string(&dif), true)
            })
        );
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ping")
        .description("Sapa EBSI dengan ramah!")
}
