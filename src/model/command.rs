use serenity::{model::prelude::application_command::CommandDataOption, builder::{CreateInteractionResponse, CreateApplicationCommand}};

pub trait CommandTrait {
    fn reg(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    fn run(options: &[CommandDataOption], response: &mut CreateInteractionResponse);
}
