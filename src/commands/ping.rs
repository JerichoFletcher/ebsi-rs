use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Duar!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ping")
        .description("Sapa EBSI dengan ramah!")
}