use serenity::{builder::CreateEmbed, model::Timestamp, utils::Color};

pub fn template_ok(e: &mut CreateEmbed, title: &str) {
    e
        .title(title)
        .color(Color::DARK_GREEN)
        .timestamp(Timestamp::now());
}
