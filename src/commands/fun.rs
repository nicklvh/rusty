use super::{Command, Context, Error};
use crate::utils::PetResponse;
use poise::{
    serenity_prelude::{Color, CreateEmbed, CreateEmbedAuthor},
    CreateReply,
};
use reqwest::get;

/// Shows a cute cat! 😻
#[poise::command(slash_command)]
async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    let cat_api_response = get("https://api.thecatapi.com/v1/images/search")
        .await?
        .json::<Vec<PetResponse>>()
        .await?;

    let embed_author_builder =
        CreateEmbedAuthor::new("Here's a cat 😻").icon_url(ctx.author().face());

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .image(cat_api_response[0].url.to_string())
        .color(Color::BLUE);

    let message_builder = CreateReply::default().embed(embed_builder);

    ctx.send(message_builder).await?;

    Ok(())
}

/// Shows a funny dog! 🐶
#[poise::command(slash_command)]
async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    let dog_api_response = get("https://api.thedogapi.com/v1/images/search")
        .await?
        .json::<Vec<PetResponse>>()
        .await?;

    let embed_author_builder =
        CreateEmbedAuthor::new("Here's a dog 🐶").icon_url(ctx.author().face());

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .image(dog_api_response[0].url.to_string())
        .color(Color::BLUE);

    let message_builder = CreateReply::default().embed(embed_builder);

    ctx.send(message_builder).await?;

    Ok(())
}

/// Shows a smart duck! 🦆
#[poise::command(slash_command)]
async fn duck(ctx: Context<'_>) -> Result<(), Error> {
    let duck_api_response = reqwest::get("https://random-d.uk/api/v2/quack")
        .await?
        .json::<PetResponse>()
        .await?;

    let embed_author_builder =
        CreateEmbedAuthor::new("Here's a duck 🦆").icon_url(ctx.author().face());

    let embed_builder = CreateEmbed::new()
        .author(embed_author_builder)
        .image(duck_api_response.url.to_string())
        .color(Color::BLUE);

    let message_builder = CreateReply::default().embed(embed_builder);

    ctx.send(message_builder).await?;

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [cat(), dog(), duck()]
}
