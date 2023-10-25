use serenity::{builder::CreateEmbed, model::Timestamp, utils::Color};

use crate::bot::Bot;

pub fn template_ok(e: &mut CreateEmbed, title: &str) {
    e
        .title(title)
        .thumbnail(Bot::self_user().as_ref().unwrap().avatar_url().unwrap())
        .color(Color::DARK_GREEN)
        .timestamp(Timestamp::now());
}
