use std::error::Error;
use regex::CaptureMatches;
use serenity:: {
    model::prelude::Message,
    prelude::*,
};
use crate::commands::{Command, Info};
use crate::util::{join_args, IsOriginal};
use crate::handlers::YHandler;

use itertools::Itertools;

pub struct PingCommand;

impl Command for PingCommand {
    fn get_info(&self) -> Info {
        Info {
            names: vec!["ping", "pong"],
        }
    }

    fn run(&self, ctx: &Context, msg: &Message, _args: CaptureMatches, _handler: &YHandler) -> Result<(), Box<dyn Error>> {
        msg.channel_id.say(&ctx.http, "Pong!")?;
        Ok(())
    }
}

pub struct SayCommand;

impl Command for SayCommand {
    fn get_info(&self) -> Info {
        Info {
            names: vec!["say", "speak", "echo"],
        }
    }

    fn run(&self, ctx: &Context, msg: &Message, args: CaptureMatches, _handler: &YHandler) -> Result<(), Box<dyn Error>> {
        let content = join_args(args).trim_start().to_string();
        msg.channel_id.say(&ctx.http, content)?;
        Ok(())
    }
}

pub struct HelpCommand;

impl Command for HelpCommand {
    fn get_info(&self) -> Info {
        Info {
            names: vec!["help", "commands", "?"],
        }
    }

    fn run(&self, ctx: &Context, msg: &Message, _args: CaptureMatches, handler: &YHandler) -> Result<(), Box<dyn Error>> {
        let mut commands = handler.commands.values()
            .filter(|command: &&IsOriginal| command.1)
            .map(|command: &IsOriginal| command.0.get_info().names.get(0).cloned().unwrap());
        let message = format!("My commands are: {}", commands.join(", "));
        msg.channel_id.say(&ctx.http, message)?;
        Ok(())
    }
}