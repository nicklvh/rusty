use crate::structs::{DbConfig, Guild, Infraction, InfractionType, PostgresContainer};
use poise::serenity_prelude::Context;
use sqlx::{migrate, PgPool, Pool, Postgres};
use tracing::{error, info};

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

pub async fn get_pool(ctx: &Context) -> Pool<Postgres> {
    ctx.data
        .read()
        .await
        .get::<PostgresContainer>()
        .unwrap()
        .clone()
}

async fn guild_exists(pool: &PgPool, guild_id: &String) -> bool {
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

pub async fn insert_guild(ctx: &Context, guild: &Guild) {
    let pool = &get_pool(ctx).await;

    if guild_exists(pool, &guild.id).await {
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

    info!("Inserted guild {}", guild.id);
}

pub async fn get_guild(ctx: &Context, guild_id: &String) -> Option<Guild> {
    let pool = &get_pool(ctx).await;

    let result = sqlx::query_as!(
        Guild,
        "SELECT id, mod_id, audit_id, welcome_id FROM guilds WHERE id = $1",
        guild_id
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(guild) => Some(guild),
        Err(_) => None,
    }
}

pub async fn insert_infraction(ctx: &Context, infraction: &Infraction) {
    let pool = &get_pool(ctx).await;

    if let Err(e) = sqlx::query_as!(Infraction,
        r#"INSERT INTO infractions (guild_id, member_id, moderator_id, reason, infraction_type) VALUES ($1, $2, $3, $4, $5)"#,
        infraction.guild_id,
        infraction.member_id,
        infraction.moderator_id,
        infraction.reason,
        infraction.infraction_type as InfractionType
    )
    .execute(pool)
    .await {
        error!("Error inserting infraction: {}", e);
        return;
    }

    info!("Inserted infraction for {}", infraction.member_id);
}

pub async fn get_infractions(pool: &PgPool, guild_id: &str, member_id: &str) -> Vec<Infraction> {
    let result = sqlx::query_as!(
        Infraction,
        r#"SELECT guild_id, member_id, moderator_id, reason, infraction_type as "infraction_type: InfractionType", created_at FROM infractions WHERE guild_id = $1 AND member_id = $2"#,
        guild_id,
        member_id
    )
    .fetch_all(pool)
    .await;

    match result {
        Ok(infractions) => infractions,
        Err(_) => Vec::new(),
    }
}
