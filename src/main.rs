use std::env;
use dotenv::dotenv;
use serenity::{
    prelude::*,
    client::validate_token,
};
use rust_bot::commands::premade::{PingCommand, SayCommand, HelpCommand};
use rust_bot::handlers::YHandler;

fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let prefix = env::var("DISCORD_PREFIX")
        .expect("Expected a prefix in the environment");
    validate_token(&token)
        .expect("Expected a valid token");
    let handler = YHandler::new(&prefix, vec![&PingCommand, &SayCommand, &HelpCommand]);
    let mut client = Client::new(&token, handler)
        .expect("Err creating client");
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
