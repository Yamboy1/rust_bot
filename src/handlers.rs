use std::error::Error;
use indexmap::IndexMap;
use regex::{Regex, escape as regex_escape};
use serenity::{
    model::prelude::{Message, Ready},
    prelude::*,
};
use crate::commands::Command;
use crate::util::IsOriginal;

pub struct YHandler {
    pub prefix: String,
    pub regex: Regex,
    pub commands: IndexMap<&'static str, IsOriginal>
}

impl YHandler {
    pub fn new(prefix: impl AsRef<str>, commands: Vec<&'static (dyn Command + Sync)>) -> Self {
        let prefix = prefix.as_ref();
        Self {
            prefix: prefix.to_string(),
            regex: Regex::new(&r"(^prefix|\s+)(\S+)".replace("prefix", &regex_escape(prefix))).expect("Invalid regex"),
            commands: command_hashmap(commands)
        }
    }
}

fn command_hashmap(vec: Vec<&'static (dyn Command + Sync)>) -> IndexMap<&'static str, IsOriginal> {
    let mut hashmap = IndexMap::new();
    for command in vec {
        let mut iter = command.get_info().names.into_iter();
        hashmap.insert(iter.next().unwrap(), IsOriginal(command, true));
        hashmap.extend(iter.map(|name| (name, IsOriginal(command as &(dyn Command + Sync), false))));
    }
    hashmap
}

impl EventHandler for YHandler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return }
        if let Err(why) = command_handler(ctx, msg, self) {
            println!("Error when responding to message: {}", why);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.tag());
    }
}

pub fn command_handler(ctx: Context, msg: Message, handler: &YHandler) -> Result<(), Box<dyn Error>> {
    let command_regex = Regex::new(&r"^prefix(\S+)".replace("prefix", &regex_escape(&handler.prefix))).unwrap();
    let rest_regex = Regex::new(r"(\s+)*(\S+)").unwrap();
    if let Some(c) = command_regex.captures(&msg.content) {
        let command_name = c.get(1).expect("here").as_str();
        let rest = command_regex.replace(&msg.content, "");
        let rest_iter = rest_regex.captures_iter(&rest);
        handler.commands.get(command_name).map_or(Ok(()), |command: &IsOriginal| command.0.run(&ctx, &msg, rest_iter, handler))?;
    }
    Ok(())
}