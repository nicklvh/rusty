use crate::{
    structs::{Command, Context, Error, InfractionType},
    utils::{get_member, handle_moderation, manageable},
};
use poise::{
    serenity_prelude::{Color, CreateEmbed, CreateEmbedAuthor, User},
    CreateReply,
};

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
    let user_member = get_member(ctx, user.id).await;
    let author_member = ctx.author_member().await.unwrap().into_owned();
    let bot_user_id = { ctx.cache().current_user().id };
    let bot_member = get_member(ctx, bot_user_id).await;
    let can_manage = manageable(ctx, &author_member, &user_member).await;
    let can_i_manage = manageable(ctx, &bot_member, &user_member).await;

    let string: String;

    if !can_manage && !can_i_manage {
        string = String::from("you and I");
    } else if !can_manage {
        string = String::from("you");
    } else {
        string = String::from("me");
    };

    if !can_manage || !can_i_manage {
        ctx.send(CreateReply::default().embed(CreateEmbed::new().author(CreateEmbedAuthor::new("Error").icon_url(user.face())).description(format!("<@{}> has a higher role than {}, or is the owner of the server, so I cannot ban them.", user.id, string)).color(Color::BLUE))).await?;

        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    handle_moderation(ctx, InfractionType::Ban, &user, &reason)
        .await
        .expect_err("error handling ban");

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
    let user_member = get_member(ctx, user.id).await;
    let author_member = ctx.author_member().await.unwrap().into_owned();
    let bot_user_id = { ctx.cache().current_user().id };
    let bot_member = get_member(ctx, bot_user_id).await;
    let can_manage = manageable(ctx, &author_member, &user_member).await;
    let can_i_manage = manageable(ctx, &bot_member, &user_member).await;

    let string: String;

    if !can_manage && !can_i_manage {
        string = String::from("you and I");
    } else if !can_manage {
        string = String::from("you");
    } else {
        string = String::from("me");
    };

    if !can_manage || !can_i_manage {
        ctx.send(CreateReply::default().embed(CreateEmbed::new().author(CreateEmbedAuthor::new("Error").icon_url(user.face())).description(format!("<@{}> has a higher role than {}, or is the owner of the server, so I cannot ban them.", user.id, string)).color(Color::BLUE))).await?;

        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    handle_moderation(ctx, InfractionType::Kick, &user, &reason)
        .await
        .expect_err("error handling kick");

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
    let user_member = get_member(ctx, user.id).await;
    let author_member = ctx.author_member().await.unwrap().into_owned();
    let bot_user_id = { ctx.cache().current_user().id };
    let bot_member = get_member(ctx, bot_user_id).await;
    let can_manage = manageable(ctx, &author_member, &user_member).await;
    let can_i_manage = manageable(ctx, &bot_member, &user_member).await;

    let string: String;

    if !can_manage && !can_i_manage {
        string = String::from("you and I");
    } else if !can_manage {
        string = String::from("you");
    } else {
        string = String::from("me");
    };

    if !can_manage || !can_i_manage {
        ctx.send(CreateReply::default().embed(CreateEmbed::new().author(CreateEmbedAuthor::new("Error").icon_url(user.face())).description(format!("<@{}> has a higher role than {}, or is the owner of the server, so I cannot mute them.", user.id, string)).color(Color::BLUE))).await?;

        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    handle_moderation(ctx, InfractionType::Mute, &user, &reason)
        .await
        .expect_err("error handling mute");

    // user_member
    //     .disable_communication_until_datetime(ctx, Utc)
    //     .await?;

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [ban(), kick(), mute()]
}
