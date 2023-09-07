use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};
use crate::api::calculator::evaluate;

pub async fn run(options: &[CommandDataOption]) -> String {
    let expr = options
        .get(0)
        .expect("No expression provided")
        .resolved
        .as_ref()
        .expect("Expected valid object");

    if let CommandDataOptionValue::String(expression) = expr {
        let result = evaluate(expression);
    
        format!("{} = {}", expression, result)
    } else {
        "Invalid expression".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("calc")
        .description("Perform calculations")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("expression")
                .description("Mathematical expression to calculate")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
