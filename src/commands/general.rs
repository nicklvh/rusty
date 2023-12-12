use poise::{
    serenity_prelude::{Color, CreateEmbed, CreateEmbedAuthor, User},
    CreateReply,
};

use crate::utils::{Command, Context, Error};

/// Shows yours or another user's avatar! 🖼️
#[poise::command(slash_command)]
async fn avatar(
    ctx: Context<'_>,
    #[description = "The user to show"] user: Option<User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or(ctx.author());

    let embed_author_builder = CreateEmbedAuthor::new(format!("{}'s avatar", user.name))
        .icon_url(user.face())
        .url(user.face());

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .image(user.face())
        .color(Color::BLUE);

    let reply_builder = CreateReply::default().embed(embed_builder);

    ctx.send(reply_builder).await?;

    Ok(())
}

pub fn commands() -> [Command; 1] {
    [avatar()]
}
