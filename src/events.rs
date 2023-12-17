use crate::{
    database::{get_pool, insert_guild},
    structs::Guild as DBGuild,
};
use poise::serenity_prelude::{async_trait, Context, EventHandler, Guild, Ready};
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _: Option<bool>) {
        let id = guild.id.to_string();

        info!("Joined guild {}", guild.name);

        let guild = DBGuild {
            id,
            mod_id: None,
            audit_id: None,
            welcome_id: None,
        };

        insert_guild(&get_pool(&ctx).await, &guild).await;
    }
}
