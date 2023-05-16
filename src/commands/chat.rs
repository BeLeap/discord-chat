use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::command::{CommandOptionType, CommandType},
};

pub fn run() -> String {
    "chit".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("chat")
        .description("Chat with bard")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("instruction")
                .description("Instruction for bard")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
