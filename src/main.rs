use anyhow::anyhow;
use futures::future::join_all;
use lazy_static::lazy_static;
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
use shuttle_secrets::SecretStore;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod api;
mod commands;

lazy_static! {
    pub static ref SECRETS: Mutex<HashMap<&'static str, String>> = Mutex::new(HashMap::new());
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let command_name = &command.data.name;
            println!("Received command: {:#?}", command_name);

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await
            {
                eprintln!("Cannot respond to application command: {}", e);
            }

            let content = match command_name.as_str() {
                "chat" => commands::chat::run(&command.data.options).await,
                _ => "Unknown Command".to_string(),
            };

            if let Err(e) = command
                .edit_original_interaction_response(&ctx.http, |response| response.content(content))
                .await
            {
                eprintln!("Cannot edit response: {}", e);
            }
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected", ready.user.name);

        let ctx = Arc::new(ctx);

        let register_fns = [commands::chat::register];
        let command_register_futures = register_fns
            .map(|register_fn| Command::create_global_application_command(&ctx.http, register_fn));
        let commands: Vec<Result<Command, Error>> = join_all(command_register_futures).await;

        println!(
            "Registered command: {:#?}",
            commands
                .iter()
                .map(|command| command.as_ref().map(|command| &command.name))
                .collect::<Vec<Result<&String, &Error>>>()
        );
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let bard_api_token = if let Some(value) = secret_store.get("BARD_API_TOKEN") {
        value
    } else {
        return Err(anyhow!("'BARD_API_TOKEN' not found").into());
    };
    SECRETS
        .lock()
        .unwrap()
        .insert("BARD_API_TOKEN", bard_api_token);

    let discord_token = if let Some(value) = secret_store.get("DISCORD_TOKEN") {
        value
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' not found").into());
    };

    let client = Client::builder(&discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    Ok(client.into())
}
