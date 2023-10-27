use serenity::{builder::CreateEmbed, model::Timestamp, utils::Color};

use crate::bot::Bot;

pub fn template_ok(e: &mut CreateEmbed, title: &str) {
    e
        .title(title)
        .thumbnail(Bot::user_avatar_url())
        .color(Color::DARK_GREEN)
        .timestamp(Timestamp::now());
}

// pub fn template_err(e: &mut CreateEmbed, title: &str) {
//     template_ok(e, title);
//     e
//         .color(Color::RED);
// }
