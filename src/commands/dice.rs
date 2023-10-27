use serenity::builder::{CreateApplicationCommand, CreateInteractionResponse};
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::{
    application_command::CommandDataOption,
    command::CommandOptionType,
    InteractionResponseType,
};

use crate::model::command::CommandTrait;
use crate::utils::embed;

pub struct PingCommand;

impl CommandTrait for PingCommand {
    fn reg(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("dadu")
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

    fn run(options: &[CommandDataOption], response: &mut CreateInteractionResponse) {
        if let CommandDataOptionValue::Integer(n) = options
            .get(0).expect("No dice <n> option").resolved.as_ref().expect("No dice <n> value") {
                let roll = dice::roll(n.unsigned_abs());

                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message
                        .embed(|e| {
                            embed::template_ok(e, "Dadu!");
                            e
                                .description("Nilai dadu kamu adalah:")
                                .field(format!(":game_die: {roll}"), "\u{200E}", false)
                        })
                    );
            }
    }
}
