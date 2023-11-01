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
pub struct HelpCommand;

impl CommandTrait for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }
    
    fn run(&self, bot: &Bot, options: &[CommandDataOption], response: &mut CreateInteractionResponse) {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message
                .embed(|e| {
                    let map = bot.cmdinfo_map.lock().unwrap();

                    if let Some(option) = options.get(0) {
                        if let CommandDataOptionValue::String(name) = option.resolved.as_ref().unwrap() {
                            match map.get(name) {
                                Some(cmd) => {
                                    let mut usage_str = format!("/{name}");
                                    let mut opt_desc_str = String::new();
                                    for option in &cmd.options {
                                        let opt_str = if option.required {
                                            format!(" <{}>", option.name)
                                        } else {
                                            format!(" [{}]", option.name)
                                        };
                                        usage_str.push_str(opt_str.as_str());
                                        opt_desc_str.push_str(format!("`{}`: {}\n", option.name, option.description).as_str());
                                    }
                                    opt_desc_str.pop().unwrap_or_default();

                                    embed::template_ok(e, bot, format!("`{name}`").as_str());
                                    e
                                        .description(&cmd.description)
                                        .field("Usage:", format!("`{usage_str}`\n{opt_desc_str}"), false)

                                },
                                None => {
                                    embed::template_err(e, bot, "Help!");
                                    e
                                        .description(format!("Tidak ada *command* dengan nama `{name}`!\nLihat daftar *command* dengan `/{}`.", self.name()))
                                }
                            }
                        } else {
                            embed::template_err(e, bot, "Help!");
                            e
                                .description("Gagal melakukan *parsing* argumen :pensive:")
                        }
                    } else {
                        embed::template_ok(e, bot, "Help!");
                        e
                            .description("Berikut adalah daftar semua *command* yang tersedia:");
                        for cmd in map.values() {
                            e.field(format!("`{}`", cmd.name), &cmd.description, false);
                        }

                        e
                    }
                })
            );
    }
    
    fn reg<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command
            .name(self.name())
            .description("Menampilkan informasi command yang tersedia.")
            .create_option(|option| option
                .name("command")
                .description("Nama command")
                .kind(CommandOptionType::String)
                .required(false)
            )
    }
}