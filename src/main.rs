#![warn(clippy::pedantic)]

mod commands;
mod database;
mod events;
mod structs;
mod utils;

use crate::{
    commands::commands,
    database::connect,
    events::Handler,
    structs::{Data, DbConfig, PostgresContainer, ShardManagerContainer},
};
use dotenvy::dotenv;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use std::env::var;
use structs::ReqwestClientContainer;
use tracing::{error, info};

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
        commands: commands(),
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

    let token = std::env::var("TOKEN").expect("missing TOKEN");

    let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILDS;

    let framework = poise::Framework::builder()
        .options(options)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let reqwest_client = reqwest::Client::new();

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<PostgresContainer>(db.clone());
        data.insert::<ReqwestClientContainer>(reqwest_client.clone());
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
