use std::error::Error;
use regex::CaptureMatches;
use serenity::{
    model::prelude::Message,
    prelude::*,
};
use crate::handlers::YHandler;

pub mod premade;

pub struct Info {
    pub names: Vec<&'static str>,
}

pub trait Command {
    fn get_info(&self) -> Info;
    fn run(&self, ctx: &Context, msg: &Message, args: CaptureMatches, handler: &YHandler) -> Result<(), Box<dyn Error>>;
}
