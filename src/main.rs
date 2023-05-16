use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let discord_token = match env::var("DISCORD_TOKEN") {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };
    println!("{}", discord_token);
}
