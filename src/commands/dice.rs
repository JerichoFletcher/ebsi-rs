use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::{
    application_command::CommandDataOption,
    command::CommandOptionType,
    InteractionResponseType,
};

use crate::bot::Bot;
use crate::model::command::CommandTrait;
use crate::utils::embed;

#[derive(Clone, Copy)]
pub struct DiceCommand;

impl CommandTrait for DiceCommand {
    fn name(&self) -> &str {
        "dadu"
    }

    fn run(&self, bot: &Bot, options: &[CommandDataOption], response: &mut CreateInteractionResponse) {
        if let CommandDataOptionValue::Integer(n) = options
            .get(0).expect("No dice <n> option").resolved.as_ref().expect("No dice <n> value") {
                let roll = dice::roll(n.unsigned_abs());

                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message
                        .embed(|e| {
                            embed::template_ok(e, bot, "Dadu!");
                            e
                                .description(format!("Melempar D-{n}! Hasilnya adalah:"))
                                .field(format!(":game_die: {roll}"), "\u{200E}", false)
                        })
                    );
            }
    }
    
    fn reg<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command
            .name(self.name())
            .description("Lempar dadu!")
            .create_option(|option| option
                .name("sisi")
                .description("Banyak sisi dari dadu")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(u32::MAX)
                .required(true)
            )
    }
}
