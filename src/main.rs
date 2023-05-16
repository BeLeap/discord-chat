use dotenvy::dotenv;
use serenity::{prelude::GatewayIntents, Client};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let discord_token = match env::var("DISCORD_TOKEN") {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };

    let mut client = Client::builder(&discord_token, GatewayIntents::empty())
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        eprintln!("Client error: {:?}", e);
    }
}
