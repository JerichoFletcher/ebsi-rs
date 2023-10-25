use std::ops::Add;
use std::time::Duration;

use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::Timestamp;
use serenity::model::prelude::{application_command::CommandDataOption, InteractionResponseType};

use crate::bot::Bot;
use crate::utils::embed;

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
                    .field("Online sejak", Bot::start_timestamp().unwrap(), true)
                    .field("\u{200E}", "\u{200E}", true)
                    .field("Online selama", duration_to_string(&dif), true)
            })
        );
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ping")
        .description("Sapa EBSI dengan ramah!")
}

fn duration_to_string(d: &Duration) -> String {
    let mut str = String::new();

    let raw = d.as_secs();
    let days = raw / (60 * 60 * 24);
    let hours = (raw % (60 * 60 * 24)) / (60 * 60);
    let minutes = (raw % (60 * 60)) / 60;
    let seconds = raw % 60;

    if days > 0 { str = str.add(format!("{days} hari ").as_str()); }
    if hours > 0 { str = str.add(format!("{hours} jam ").as_str()); }
    if minutes > 0 { str = str.add(format!("{minutes} menit ").as_str()); }
    if days == 0 && hours == 0 { str = str.add(format!("{seconds} detik").as_str()); }

    str
}
