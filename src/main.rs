use dotenvy::dotenv;
use futures::future::join_all;
use serenity::{
    async_trait,
    model::prelude::{
        command::Command,
        interaction::{Interaction, InteractionResponseType},
        Ready,
    },
    prelude::{Context, EventHandler, GatewayIntents},
    Client, Error,
};
use std::{env, sync::Arc};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command: {:#?}", command);

            let content = match command.data.name.as_str() {
                "chat" => commands::chat::run(),
                _ => "Unknown Command".to_string(),
            };

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(content))
                })
                .await
            {
                eprintln!("Cannot respond to application command: {}", e);
            };
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected", ready.user.name);

        let ctx = Arc::new(ctx);

        let register_fns = [commands::chat::register];
        let command_register_futures = register_fns
            .map(|register_fn| Command::create_global_application_command(&ctx.http, register_fn));
        let commands: Vec<Result<Command, Error>> = join_all(command_register_futures).await;

        println!("Registered command: {:#?}", commands);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let discord_token = match env::var("DISCORD_TOKEN") {
        Ok(value) => value,
        Err(e) => panic!("{}", e),
    };

    let mut client = Client::builder(&discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        eprintln!("Client error: {:?}", e);
    }
}
