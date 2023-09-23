use std::collections::HashMap;
use std::sync::{Arc, OnceLock, Mutex};

use async_trait::async_trait;
use serenity::builder::{CreateApplicationCommands, CreateApplicationCommand};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use super::commands::ping;
use super::commands::record::record;

#[async_trait]
pub trait SlashCommand {
    fn get_name(&self) -> String;
    fn regist_command(&self) -> Box<dyn FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand>;
    async fn handle_command(&self, ctx :Context, command: ApplicationCommandInteraction);
}

pub fn regist_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    let slash_commands: Vec<Arc<dyn SlashCommand + Sync + Send>> = vec![
            Arc::new(ping::new()),
            Arc::new(record::new())
        ];

    for sc in slash_commands {
        commands.create_application_command(sc.regist_command());
        hashmap().lock().unwrap().insert(sc.get_name(), sc);
    };

    commands
}

pub async fn handle_commands(ctx :Context, command: ApplicationCommandInteraction) {
    let name = &command.data.name;

    println!("command {} in", name);

    let sc = match hashmap().lock().unwrap().get(name) {
        Some(c) => Arc::clone(c),
        None => return println!("No RES"),
    };
    
    sc.handle_command(ctx, command).await;
}

fn hashmap() -> &'static Mutex<HashMap<String, Arc<dyn SlashCommand + Sync + Send>>> {
    static SLASH_COMMANDS: OnceLock<Mutex<HashMap<String, Arc<dyn SlashCommand + Sync + Send>>>> = OnceLock::new();
    SLASH_COMMANDS.get_or_init(|| HashMap::new().into())
}
