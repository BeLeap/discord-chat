use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};

use crate::api::chattable::Chattable;
use crate::api::cohere::Cohere;

pub async fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("No instruction provided")
        .resolved
        .as_ref()
        .expect("Expected valid object");

    if let CommandDataOptionValue::String(instruction) = option {
        let hf_alpaca = Cohere::new();

        match hf_alpaca.chat(instruction.to_string()).await {
            Ok(response) => response,
            Err(e) => format!("Failed to chat: {}", e).to_string(),
        }
    } else {
        "Invalid instruction".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("chat")
        .description("Chat with LLM")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("instruction")
                .description("Instruction for LLM")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
