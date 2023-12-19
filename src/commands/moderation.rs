use crate::{
    structs::{Command, Context, Error, InfractionType},
    utils::{get_member, handle_moderation, manageable, send_error_msg},
};
use poise::serenity_prelude::User;
use tracing::error;

async fn check_manageable(ctx: Context<'_>, user: &User, infraction_type: InfractionType) -> bool {
    let user_member = get_member(ctx, user.id).await;
    let author_member = ctx.author_member().await.unwrap().into_owned();
    let bot_user_id = { ctx.cache().current_user().id };
    let bot_member = get_member(ctx, bot_user_id).await;
    let can_manage = manageable(ctx, &author_member, &user_member).await;
    let can_i_manage = manageable(ctx, &bot_member, &user_member).await;

    if !can_manage || !can_i_manage {
        let string: &str;

        if !can_manage && !can_i_manage {
            string = "you and I";
        } else if !can_manage {
            string = "you";
        } else {
            string = "me";
        };

        let infraction_type = match infraction_type {
            InfractionType::Ban => "ban",
            InfractionType::Kick => "kick",
            InfractionType::Mute => "mute",
            InfractionType::Warn => "warn",
        };

        send_error_msg(ctx, &format!("<@{}> has a higher role than {}, or is the owner of the server, so I cannot {} them.", user.id, string, infraction_type)).await;
        return false;
    }

    true
}

/// Ban a member! 🔨
#[poise::command(
    slash_command,
    default_member_permissions = "BAN_MEMBERS",
    required_bot_permissions = "BAN_MEMBERS",
    required_permissions = "BAN_MEMBERS",
    guild_only = true
)]
async fn ban(
    ctx: Context<'_>,
    #[description = "The user to ban"] user: User,
    #[description = "The reason for banning this user"] reason: Option<String>,
) -> Result<(), Error> {
    if !check_manageable(ctx, &user, InfractionType::Ban).await {
        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    if let Err(e) = handle_moderation(ctx, InfractionType::Ban, &user, &reason).await {
        error!("Error: {}", e);
        send_error_msg(ctx, "Error handling ban").await;
        return Ok(());
    }

    ctx.guild_id()
        .unwrap()
        .ban_with_reason(ctx, &user.id, 7, reason.as_str())
        .await?;

    Ok(())
}

/// Kick a member! 🔨
#[poise::command(
    slash_command,
    default_member_permissions = "KICK_MEMBERS",
    required_bot_permissions = "KICK_MEMBERS",
    required_permissions = "KICK_MEMBERS",
    guild_only = true
)]
async fn kick(
    ctx: Context<'_>,
    #[description = "The user to kick"] user: User,
    #[description = "The reason for kicking this user"] reason: Option<String>,
) -> Result<(), Error> {
    if !check_manageable(ctx, &user, InfractionType::Kick).await {
        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    if let Err(e) = handle_moderation(ctx, InfractionType::Kick, &user, &reason).await {
        error!("Error: {}", e);
        send_error_msg(ctx, "Error handling kick").await;
        return Ok(());
    }

    ctx.guild_id()
        .unwrap()
        .kick_with_reason(ctx, &user.id, reason.as_str())
        .await?;

    Ok(())
}

/// Mute a member! 🔨
#[poise::command(
    slash_command,
    default_member_permissions = "MODERATE_MEMBERS",
    required_bot_permissions = "MODERATE_MEMBERS",
    required_permissions = "MODERATE_MEMBERS",
    guild_only = true
)]
async fn mute(
    ctx: Context<'_>,
    #[description = "The user to mute"] user: User,
    #[description = "The reason for muting this user"] reason: Option<String>,
) -> Result<(), Error> {
    if !check_manageable(ctx, &user, InfractionType::Mute).await {
        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    if let Err(e) = handle_moderation(ctx, InfractionType::Mute, &user, &reason).await {
        error!("Error: {}", e);
        send_error_msg(ctx, "Error handling mute").await;
        return Ok(());
    }

    // @TODO: Add mute timing

    // user_member
    //     .disable_communication_until_datetime(ctx, Utc)
    //     .await?;

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [ban(), kick(), mute()]
}
