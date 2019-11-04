use std::{env};
use regex::{Regex};

use serenity::{
    model::channel::Message,
    prelude::*,
};

use crate::commands::*;
use std::collections::HashMap;

pub fn command_handler(ctx: Context, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    let prefix = env::var("DISCORD_PREFIX")
        .expect("Expected a prefix in the environment");
    let re = Regex::new(&r"(^prefix|\s+)(\S+)".replace("prefix", &prefix))?;
    let mut captured_iter = re.captures_iter(&msg.content);

    if let Some(c) = captured_iter.next() {
        let command_name = c.get(2).unwrap().as_str();
        let hashmap: HashMap<&str, &dyn Command> = command_hashmap![PingCommand, SayCommand];
        hashmap.get(command_name).map_or(Ok(()), |command| command.run(&ctx, &msg, captured_iter))?;
    }
    Ok(())
}

pub fn message_handler(ctx: Context, msg: Message) -> Result<(), Box<dyn std::error::Error>> {
    command_handler(ctx, msg)
}
