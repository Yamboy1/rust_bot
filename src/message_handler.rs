use serenity::{
    model::{channel::Message},
    prelude::*,
    Error,
};

pub fn message_handler(ctx: Context, msg: Message) -> Result<(), Error> {
    if msg.content == "!ping" {
        msg.channel_id.say(&ctx.http, "Pong!")?;
    } else if msg.content.starts_with("!say ") {
        let response = msg.content.trim_start_matches("!say").trim_start();
        msg.channel_id.say(&ctx.http, response)?;
    }
    Ok(())
}