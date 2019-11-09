use std::error::Error;
use std::collections::HashMap;
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
    fn run(&self, ctx: &Context, msg: &Message, _args: CaptureMatches, _handler: &YHandler) -> Result<(), Box<dyn Error>>;
}
