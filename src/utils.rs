use crate::{
    database::{get_guild, insert_guild, insert_infraction},
    structs::{Context, Error, Guild, Infraction, InfractionType, ReqwestClientContainer},
};
use poise::{
    serenity_prelude::{
        ChannelId, Color, Context as SerenityContext, CreateEmbed, CreateEmbedAuthor,
        CreateMessage, Member, RoleId, Timestamp, User, UserId,
    },
    CreateReply,
};
use reqwest::Client;
use tracing::error;

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
    mod_type: InfractionType,
    user: &User,
    reason: &String,
) -> Result<(), Error> {
    send_mod_msg_to_user(ctx, &mod_type, user, reason).await;
    send_mod_msg_to_channel(ctx, &mod_type, user, reason).await;

    let infraction = Infraction {
        guild_id: ctx.guild_id().unwrap().to_string(),
        member_id: user.id.to_string(),
        moderator_id: ctx.author().id.to_string(),
        reason: reason.to_string(),
        infraction_type: mod_type,
        created_at: None,
    };

    insert_guild(
        ctx.serenity_context(),
        &Guild {
            id: ctx.guild_id().unwrap().to_string(),
            mod_id: None,
            audit_id: None,
            welcome_id: None,
        },
    )
    .await;
    insert_infraction(ctx.serenity_context(), &infraction).await;

    let guild = if let Some(g) =
        get_guild(ctx.serenity_context(), &ctx.guild_id().unwrap().to_string()).await
    {
        g
    } else {
        let guild = Guild {
            id: ctx.guild_id().unwrap().to_string(),
            mod_id: None,
            audit_id: None,
            welcome_id: None,
        };

        insert_guild(ctx.serenity_context(), &guild).await;

        guild
    };

    if guild.mod_id.is_some() {
        let mod_id = guild.mod_id.unwrap();
        let modlog_id = ChannelId::from(mod_id.parse::<u64>().unwrap());

        send_mod_msg_to_modlog(&ctx, &mod_type, user, reason, modlog_id).await;
    }

    Ok(())
}

pub async fn send_mod_msg_to_user(
    ctx: Context<'_>,
    mod_type: &InfractionType,
    user: &User,
    reason: &String,
) {
    if user.bot {
        return;
    }

    let mod_type = match mod_type {
        InfractionType::Ban => "banned",
        InfractionType::Kick => "kicked",
        InfractionType::Mute => "muted",
        InfractionType::Warn => "warned",
    };

    if let Err(e) = user
        .dm(
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
        .await
    {
        error!("Error: {}", e);
    }
}

pub async fn send_mod_msg_to_channel(
    ctx: Context<'_>,
    mod_type: &InfractionType,
    user: &User,
    reason: &String,
) {
    let mod_type = match mod_type {
        InfractionType::Ban => "Banned",
        InfractionType::Kick => "Kicked",
        InfractionType::Mute => "Muted",
        InfractionType::Warn => "Warned",
    };

    if let Err(e) = ctx
        .send(
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
        .await
    {
        error!("Error: {}", e);
    }
}

pub async fn send_mod_msg_to_modlog(
    ctx: &Context<'_>,
    mod_type: &InfractionType,
    user: &User,
    reason: &String,
    modlog_id: ChannelId,
) {
    let mod_type = match mod_type {
        InfractionType::Ban => "Banned",
        InfractionType::Kick => "Kicked",
        InfractionType::Mute => "Muted",
        InfractionType::Warn => "Warned",
    };

    let channel = ctx.http().get_channel(modlog_id).await;

    if channel.is_err() {
        return error!("Modlog channel not found");
    }

    if let Err(e) = ctx
        .send(
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
        .await
    {
        error!("Error: {}", e);
    }
}

pub async fn get_reqwest_client(ctx: &SerenityContext) -> Client {
    ctx.data
        .read()
        .await
        .get::<ReqwestClientContainer>()
        .unwrap()
        .clone()
}

pub async fn send_error_msg(ctx: Context<'_>, msg: &str) {
    if let Err(e) = ctx
        .send(
            CreateReply::default().embed(
                CreateEmbed::new()
                    .author(CreateEmbedAuthor::new("Error").icon_url(ctx.author().face()))
                    .description(msg)
                    .color(Color::RED),
            ),
        )
        .await
    {
        error!("Error: {}", e);
    }
}
