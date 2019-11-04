use std::env;
use dotenv::dotenv;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    client::validate_token,
};

#[macro_use]
mod util;
mod handlers;
mod commands;

use handlers::message_handler;

struct Handler;

fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    validate_token(&token)
        .expect("Expected a valid token");

    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if let Err(why) = message_handler(ctx, msg) {
            println!("Error when responding to message: {}", why);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.tag());
    }
}
