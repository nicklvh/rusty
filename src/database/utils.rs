use crate::PostgresContainer;
use poise::serenity_prelude::Context;
use sqlx::{migrate, PgPool, Pool, Postgres};
use tracing::info;

pub struct Guild<'a> {
    pub id: &'a str,
    pub mod_id: Option<&'a str>,
    pub audit_id: Option<&'a str>,
    pub welcome_id: Option<&'a str>,
}

pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub async fn connect(db_config: &DbConfig) -> Pool<Postgres> {
    let db = sqlx::postgres::PgPoolOptions::new()
        .min_connections(5)
        .max_connections(100)
        .connect_with(
            sqlx::postgres::PgConnectOptions::new()
                .host(db_config.host.as_str())
                .port(db_config.port)
                .username(db_config.username.as_str())
                .password(db_config.password.as_str())
                .database("rusty"),
        )
        .await
        .expect("Failed to connect to database");

    migrate!().run(&db).await.expect("Failed to run migrations");

    db
}

pub async fn get_database(ctx: Context) -> Pool<Postgres> {
    ctx.data
        .read()
        .await
        .get::<PostgresContainer>()
        .unwrap()
        .clone()
}

async fn guild_exists(pool: &PgPool, guild_id: &str) -> bool {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM guilds WHERE id = $1",
        guild_id
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => row.count.unwrap_or(0) > 0,
        Err(_) => false,
    }
}

pub async fn insert_guild(pool: &PgPool, guild: Guild<'_>) {
    if guild_exists(pool, guild.id).await {
        info!("Guild {} already exists", guild.id);
        return;
    }

    sqlx::query!(
        "INSERT INTO guilds (id, mod_id, audit_id, welcome_id) VALUES ($1, $2, $3, $4)",
        guild.id,
        guild.mod_id,
        guild.audit_id,
        guild.welcome_id
    )
    .execute(pool)
    .await
    .expect("Error inserting guild");
}
