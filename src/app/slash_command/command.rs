use std::collections::HashMap;
use std::sync::{Arc, OnceLock, Mutex};

use async_trait::async_trait;
use serenity::builder::{CreateApplicationCommands, CreateApplicationCommand};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use super::commands::ping::Ping;
use super::commands::record::record::Record;

#[async_trait]
pub trait SlashCommand {
    fn get_name(&self) -> String;
    fn regist_command(&self) -> Box<dyn FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand>;
    async fn handle_command(&self, ctx :Context, command: ApplicationCommandInteraction);
}

pub fn regist_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    let slash_commands: Vec<Arc<dyn SlashCommand + Sync + Send>> = vec![
            Arc::new(Ping::new()),
            Arc::new(Record::new())
        ];

    for sc in slash_commands {
        commands.create_application_command(sc.regist_command());
        slash_command_map().lock().unwrap().insert(sc.get_name(), sc);
    };

    commands
}

pub async fn handle_commands(ctx :Context, command: ApplicationCommandInteraction) {
    let name = &command.data.name;

    println!("command {} in", name);
    
    let sc = {
        // clone 跟 unwrap 暫存的關係要拆兩行
        let slash_command_map = slash_command_map();
        let slash_command_map = slash_command_map.lock().unwrap();

        match slash_command_map.get(name) {
            Some(s) => Arc::clone(s),
            None => return println!("No RES"),
        }
    };
    
    sc.handle_command(ctx, command).await;
}

type SlashCommandMap = Arc<Mutex<HashMap<String, Arc<dyn SlashCommand + Sync + Send>>>>;

fn slash_command_map() -> SlashCommandMap {
    static SLASH_COMMANDS: OnceLock<SlashCommandMap> = OnceLock::new();
    Arc::clone(&SLASH_COMMANDS.get_or_init(|| Arc::new(Mutex::new(HashMap::new()))))
}
