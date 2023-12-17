pub mod database;

use poise::{
    serenity_prelude::{
        prelude::TypeMapKey, Color, CreateEmbed, CreateEmbedAuthor, CreateMessage, Member, RoleId,
        ShardManager, Timestamp, User, UserId,
    },
    CreateReply,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

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

#[derive(Deserialize)]
pub struct PetResponse {
    pub url: String,
}

pub enum ModerationType {
    Ban,
    Kick,
    Mute,
    Warn,
}

pub async fn manageable(ctx: Context<'_>, member: &Member, target: &Member) -> bool {
    let member_highest_role = highest_role_position(ctx, &member.roles);
    let target_highest_role = highest_role_position(ctx, &target.roles);

    if member_highest_role < target_highest_role || target.user.id == ctx.guild().unwrap().owner_id
    {
        return false;
    }

    true
}

fn highest_role_position(ctx: Context<'_>, roles: &Vec<RoleId>) -> u16 {
    let guild = ctx.guild().unwrap();
    let mut highest_role_pos = 0;

    for role_id in roles {
        let role = guild.roles.get(role_id).unwrap();

        if highest_role_pos < role.position {
            highest_role_pos = role.position;
        }
    }

    highest_role_pos
}

pub async fn get_member(ctx: Context<'_>, id: UserId) -> Member {
    let cache_member = {
        let guild = ctx.guild().unwrap();
        guild.members.get(&id).cloned()
    };

    match cache_member {
        Some(m) => m,
        None => ctx.guild_id().unwrap().member(ctx, id).await.unwrap(),
    }
}

pub async fn handle_moderation(
    ctx: Context<'_>,
    mod_type: &ModerationType,
    user: &User,
    reason: &String,
) -> Result<(), Error> {
    send_mod_msg_to_user(ctx, mod_type, user, reason)
        .await
        .expect_err(&format!("{} has their dms off", user.tag()).to_string());

    send_mod_msg_to_channel(ctx, mod_type, user, reason).await?;

    Ok(())
}

pub async fn send_mod_msg_to_user(
    ctx: Context<'_>,
    mod_type: &ModerationType,
    user: &User,
    reason: &String,
) -> Result<(), Error> {
    let mod_type = match mod_type {
        ModerationType::Ban => "banned",
        ModerationType::Kick => "kicked",
        ModerationType::Mute => "muted",
        ModerationType::Warn => "warned",
    };

    user.dm(
        ctx,
        CreateMessage::default().embed(
            CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(format!(
                        "You have been {} from {}",
                        mod_type,
                        ctx.guild().unwrap().name
                    ))
                    .icon_url(user.face()),
                )
                .field("Reason", reason.to_string(), true)
                .timestamp(Timestamp::now())
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}

pub async fn send_mod_msg_to_channel(
    ctx: Context<'_>,
    mod_type: &ModerationType,
    user: &User,
    reason: &String,
) -> Result<(), Error> {
    let mod_type = match mod_type {
        ModerationType::Ban => "Banned",
        ModerationType::Kick => "Kicked",
        ModerationType::Mute => "Muted",
        ModerationType::Warn => "Warned",
    };

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(format!("{} {}", mod_type, user.name))
                        .icon_url(user.face()),
                )
                .field("User", format!("<@{}>", user.id), true)
                .field("Reason", reason.to_string(), true)
                .timestamp(Timestamp::now())
                .color(Color::BLUE),
        ),
    )
    .await?;

    Ok(())
}
