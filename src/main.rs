#![warn(clippy::pedantic)]

mod commands;
mod utils;

use dotenv::dotenv;
use poise::serenity_prelude::{
    async_trait, ClientBuilder, Context, EventHandler, GatewayIntents, Ready,
};
use sea_orm::{ConnectOptions, Database};
use tracing::{error, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");

    let mut db_opts = ConnectOptions::new(db_url);
    db_opts
        .sqlx_logging(true)
        .max_connections(100)
        .min_connections(5)
        .set_schema_search_path("schema");

    let db = Database::connect(db_opts).await;

    match db {
        Ok(_) => {
            info!("Connected to database");
        }
        Err(err) => {
            error!("{}", err);
        }
    }

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
                Ok(utils::Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await;

    client.unwrap().start().await.unwrap();
}
