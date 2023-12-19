use poise::serenity_prelude::{async_trait, Context, EventHandler, Guild, Ready};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn guild_create(&self, _: Context, guild: Guild, _: Option<bool>) {
        info!("Joined guild {}", guild.name);
    }
}
