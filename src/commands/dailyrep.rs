use anyhow::Context as Ctx;
use serenity::{
    client::Context,
    framework::standard::{macros::*, CommandResult},
    model::channel::Message,
};

use crate::UserData;

#[command]
async fn dailyrep(ctx: &Context, msg: &Message) -> CommandResult {
    let todays_activity = ctx
        .data
        .read()
        .await
        .get::<UserData>()
        .context("Could not get userdata from database")?
        .users
        .get(&msg.author.id)
        .context("Could not get daily rep")?
        .activity[0];

    msg.channel_id
        .say(
            &ctx.http,
            format!("Your activity for today: {}", todays_activity),
        )
        .await?;
    Ok(())
}
