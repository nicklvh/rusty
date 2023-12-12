use poise::serenity_prelude::{Member, RoleId, UserId};
use serde::Deserialize;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

#[derive(Deserialize)]
pub struct PetResponse {
    pub url: String,
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
