use std::error::Error;

use serenity::{
    model::channel::Message,
    prelude::*,
};

use crate::util::join_args;
use regex::CaptureMatches;

pub struct Info {
    pub names: Vec<&'static str>,
}

pub trait Command {
    fn get_info(&self) -> Info;
    fn run(&self, ctx: &Context, msg: &Message, _args: CaptureMatches) -> Result<(), Box<dyn Error>>;
}

pub struct PingCommand;

impl Command for PingCommand {
    fn get_info(&self) -> Info {
       Info {
            names: vec!["ping", "pong"],
       }
    }

    fn run(&self, ctx: &Context, msg: &Message, _args: CaptureMatches) -> Result<(), Box<dyn Error>> {
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

    fn run(&self, ctx: &Context, msg: &Message, _args: CaptureMatches) -> Result<(), Box<dyn Error>> {
        let content = join_args(_args);
        msg.channel_id.say(&ctx.http, content)?;
        Ok(())
    }
}
