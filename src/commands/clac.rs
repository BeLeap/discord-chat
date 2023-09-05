use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};
use crate::api::calculator::evaluate;

pub async fn run(options: &[CommandDataOption]) -> String {
    let expression = if let Some(CommandDataOption {
        value: Some(CommandDataOptionValue::String(expr)),
        ..
    }) = options.get(0) {
        expr
    } else {
        return "Invalid expression".to_string();
    };

    let result = evaluate(expression);
    
    format!("{} = {}", expression, result)
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
