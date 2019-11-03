use std::env;
use dotenv::dotenv;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod message_handler;
use message_handler::message_handler;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        match message_handler(ctx, msg) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
