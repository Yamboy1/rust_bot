use serenity::{
    model::channel::Message,
    prelude::*,
    Error
};

use regex::CaptureMatches;

pub fn ping_command(ctx: &Context, msg: &Message, iter: CaptureMatches) -> Result<(), Error> {
    msg.channel_id.say(&ctx.http, "Pong!")?;
    Ok(())
}

pub fn say_command(ctx: &Context, msg: &Message, iter: CaptureMatches) -> Result<(), Error> {
    let content = iter.fold(String::from(""), |acc, x| acc + x.get( 0).unwrap().as_str());
    msg.channel_id.say(&ctx.http, content)?;
    Ok(())
}