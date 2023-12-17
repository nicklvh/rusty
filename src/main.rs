#![warn(clippy::pedantic)]

mod commands;

use dotenvy::dotenv;
use poise::serenity_prelude::{
    async_trait, ClientBuilder, Context, EventHandler, GatewayIntents, Guild, Ready,
};
use rusty::{
    database::utils::{connect, get_database, insert_guild, DbConfig, Guild as DBGuild},
    PostgresContainer, ShardManagerContainer,
};
use std::env::var;
use tracing::{error, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _: Option<bool>) {
        let id = guild.id.to_string();

        info!("Joined guild {}", guild.name);

        let guild = DBGuild {
            id: id.as_str(),
            mod_id: None,
            audit_id: None,
            welcome_id: None,
        };

        insert_guild(&get_database(ctx).await, guild).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let host = var("DATABASE_HOST").expect("missing DATABASE_HOST env");
    let port = var("DATABASE_PORT")
        .expect("missing DATABASE_PORT env")
        .parse::<u16>()
        .expect("DATABASE_PORT env is not of type u16");
    let username = var("DATABASE_USER").expect("missing DATABASE_USER env");
    let password = var("DATABASE_PASS").expect("missing DATABASE_PASS env");

    let db_config = DbConfig {
        host,
        port,
        username,
        password,
    };

    let db = connect(&db_config).await;

    let options = poise::FrameworkOptions {
        commands: commands::commands(),
        pre_command: |ctx| {
            Box::pin(async move {
                info!(
                    "Command {} was ran by user {}",
                    ctx.command().name,
                    ctx.author().tag()
                );
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                info!(
                    "Command {} has successfully finished running by user {}",
                    ctx.command().name,
                    ctx.author().tag()
                );
            })
        },
        ..Default::default()
    };

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(options)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(rusty::Data {})
            })
        })
        .build();

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<PostgresContainer>(db.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
        db.close().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
