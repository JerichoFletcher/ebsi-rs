use serenity::{model::prelude::application_command::CommandDataOption, builder::{CreateInteractionResponse, CreateApplicationCommand}};

use crate::bot::Bot;

pub trait CommandTrait: Send + Sync {
    fn name(&self) -> &str;
    fn run(&self, bot: &Bot, options: &[CommandDataOption], response: &mut CreateInteractionResponse);
    fn reg<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand;
}
