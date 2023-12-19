use crate::{
    structs::{Command, Context, Error, PetResponse},
    utils::{get_reqwest_client, send_error_msg},
};
use poise::{
    serenity_prelude::{Color, CreateEmbed, CreateEmbedAuthor},
    CreateReply,
};
use tracing::error;

/// Shows a cute cat! 😻
#[poise::command(slash_command)]
async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    let client = get_reqwest_client(ctx.serenity_context()).await;

    let request = client
        .get("https://api.thecatapi.com/v1/images/search")
        .send()
        .await;

    match request {
        Ok(res) => match res.json::<Vec<PetResponse>>().await {
            Ok(res) => {
                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::new()
                            .author(
                                CreateEmbedAuthor::new("Here's a cat! 😻")
                                    .icon_url(ctx.author().face()),
                            )
                            .image(res[0].url.to_string())
                            .color(Color::BLUE),
                    ),
                )
                .await?;
            }
            Err(e) => {
                error!("Error: {}", e);
                send_error_msg(ctx, "Error handing web request to cat API, try again later").await;
            }
        },
        Err(e) => {
            error!("Error: {}", e);
            send_error_msg(ctx, "Error handing web request to cat API, try again later").await;
        }
    }

    Ok(())
}

/// Shows a funny dog! 🐶
#[poise::command(slash_command)]
async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    let client = get_reqwest_client(ctx.serenity_context()).await;

    let request = client
        .get("https://api.thedogapi.com/v1/images/search")
        .send()
        .await;

    match request {
        Ok(res) => match res.json::<Vec<PetResponse>>().await {
            Ok(res) => {
                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::new()
                            .author(
                                CreateEmbedAuthor::new("Here's a dog! 🐶")
                                    .icon_url(ctx.author().face()),
                            )
                            .image(res[0].url.to_string())
                            .color(Color::BLUE),
                    ),
                )
                .await?;
            }
            Err(e) => {
                error!("Error: {}", e);
                send_error_msg(ctx, "Error handing web request to dog API, try again later").await;
            }
        },
        Err(e) => {
            error!("Error: {}", e);
            send_error_msg(ctx, "Error handing web request to dog API, try again later").await;
        }
    }

    Ok(())
}

/// Shows a smart duck! 🦆
#[poise::command(slash_command)]
async fn duck(ctx: Context<'_>) -> Result<(), Error> {
    let client = get_reqwest_client(ctx.serenity_context()).await;

    let request = client.get("https://random-d.uk/api/v2/quack").send().await;

    match request {
        Ok(res) => match res.json::<PetResponse>().await {
            Ok(res) => {
                ctx.send(
                    CreateReply::default().embed(
                        CreateEmbed::new()
                            .author(
                                CreateEmbedAuthor::new("Here's a duck! 🦆")
                                    .icon_url(ctx.author().face()),
                            )
                            .image(res.url.to_string())
                            .color(Color::BLUE),
                    ),
                )
                .await?;
            }
            Err(e) => {
                error!("Error: {}", e);
                send_error_msg(
                    ctx,
                    "Error handing web request to duck API, try again later",
                )
                .await;
            }
        },
        Err(e) => {
            error!("Error: {}", e);
            send_error_msg(
                ctx,
                "Error handing web request to duck API, try again later",
            )
            .await;
        }
    }

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [cat(), dog(), duck()]
}
