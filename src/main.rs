mod commands;
mod utils;
mod bot;

use anyhow::anyhow;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents =
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT |
        GatewayIntents::GUILD_EMOJIS_AND_STICKERS;

    let client = Client::builder(&token, intents)
        .event_handler(bot::Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
