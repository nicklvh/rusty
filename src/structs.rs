use poise::serenity_prelude::{prelude::TypeMapKey, ShardManager};
use serde::{Deserialize, Serialize};
use sqlx::{types::uuid::Timestamp, Pool, Postgres, Type};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PetResponse {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Type)]
#[sqlx(type_name = "infraction_type", rename_all = "lowercase")]
pub enum InfractionType {
    Ban,
    Kick,
    Mute,
    Warn,
}

pub struct Guild {
    pub id: String,
    pub mod_id: Option<String>,
    pub audit_id: Option<String>,
    pub welcome_id: Option<String>,
}

pub struct Infraction {
    pub guild_id: String,
    pub member_id: String,
    pub moderator_id: String,
    pub reason: String,
    pub infraction_type: InfractionType,
    pub created_at: Option<Timestamp>,
}

pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

pub struct ShardManagerContainer;
pub struct PostgresContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

impl TypeMapKey for PostgresContainer {
    type Value = Pool<Postgres>;
}
