use crate::utils::{get_member, manageable, Command, Context, Error};
use poise::{
    serenity_prelude::{Color, CreateEmbed, CreateEmbedAuthor, CreateMessage, Timestamp, User},
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

    if !can_manage || !can_i_manage {
        let embed_author_builder = CreateEmbedAuthor::new("Error").icon_url(user.face());

        let embed_builder = CreateEmbed::new()
            .author(embed_author_builder)
            .description(format!("<@{}> has a higher role than you or me, or is the owner of the server, so I cannot ban them.", user.id))
            .color(Color::BLUE);

        let message_builder = CreateReply::default().embed(embed_builder);

        ctx.send(message_builder).await?;

        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    let embed_author_builder = {
        let guild = ctx.guild().unwrap();
        CreateEmbedAuthor::new(format!("You have been banned from {}", guild.name))
            .icon_url(user.face())
    };

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .field("Reason", format!("<@{}>", user.id), true)
        .timestamp(Timestamp::now())
        .color(Color::BLUE);

    let message_builder = CreateMessage::default().embed(embed_builder);

    user.dm(ctx, message_builder).await?;

    ctx.guild_id()
        .unwrap()
        .ban_with_reason(ctx, &user, 7, reason)
        .await?;

    let embed_author_builder =
        CreateEmbedAuthor::new(format!("Banned {}", user.name)).icon_url(user.face());

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .field("User", format!("<@{}>", user.id), true)
        .timestamp(Timestamp::now())
        .color(Color::BLUE);

    let reply_builder = CreateReply::default().embed(embed_builder);

    ctx.send(reply_builder).await?;

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

    if !can_manage || !can_i_manage {
        let embed_author_builder = CreateEmbedAuthor::new("Error").icon_url(user.face());

        let embed_builder = CreateEmbed::new()
            .author(embed_author_builder)
            .description(format!("<@{}> has a higher role than you or me, or is the owner of the server, so I cannot kick them.", user.id))
            .color(Color::BLUE);

        let message_builder = CreateReply::default().embed(embed_builder);

        ctx.send(message_builder).await?;

        return Ok(());
    }

    let reason = reason.unwrap_or(String::from("No reason provided"));

    let embed_author_builder = CreateEmbedAuthor::new(format!(
        "You have been kicked from {}",
        ctx.guild().unwrap().name
    ))
    .icon_url(user.face());

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .field("Reason", format!("<@{}>", user.id), true)
        .timestamp(Timestamp::now())
        .color(Color::BLUE);

    let message_builder = CreateMessage::default().embed(embed_builder);

    user.dm(ctx, message_builder).await?;

    ctx.guild_id()
        .unwrap()
        .kick_with_reason(ctx, &user.id, reason.as_str())
        .await?;

    let embed_author_builder =
        CreateEmbedAuthor::new(format!("Kicked {}", user.name)).icon_url(user.face());

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .field("User", format!("<@{}>", user.id), true)
        .timestamp(Timestamp::now())
        .color(Color::BLUE);

    let reply_builder = CreateReply::default().embed(embed_builder);

    ctx.send(reply_builder).await?;

    Ok(())
}

pub fn commands() -> [Command; 2] {
    [ban(), kick()]
}
