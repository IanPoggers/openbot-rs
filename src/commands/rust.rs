use serenity::client::Context;
use serenity::{
    framework::standard::{macros::*, CommandResult},
    http::CacheHttp,
    model::channel::Message,
};

#[command]
pub async fn rust(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx.http(), "This is an official rust programming language server. Discussion of all other programming languages is prohibited").await?;
    Ok(())
}
