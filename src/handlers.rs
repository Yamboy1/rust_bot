use std::{env, error::*};
use regex::{Regex, Captures, Match};

use serenity::{
    model::channel::Message,
    prelude::*,
    Error,
};

use crate::commands::*;

pub fn command_handler(ctx: Context, msg: Message) -> Result<(), Error> {
    let prefix = env::var("DISCORD_PREFIX")
        .expect("Expected a prefix in the environment");

    let re = Regex::new(&r"(^prefix|\s+)(\S+)".replace("prefix", &prefix))
        .expect("Invalid regular expression");

    let mut captured_iter = re.captures_iter(&msg.content);

    let next = captured_iter.next();
    if next.is_none() {
        Ok(())
    } else {
        let command_name = next.unwrap().get(2).unwrap().as_str();

        match command_name {
            "ping" => ping_command(&ctx, &msg, captured_iter),
            "say" => say_command(&ctx, &msg, captured_iter),
            _ => Ok(()),
        }
    }
}

pub fn message_handler(ctx: Context, msg: Message) -> Result<(), Error> {
    command_handler(ctx, msg)
}