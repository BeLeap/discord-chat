use serenity::builder::CreateApplicationCommand;

pub fn run() -> String {
    "chit".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("chat")
}
