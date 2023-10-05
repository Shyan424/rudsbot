use std::sync::OnceLock;

use serenity::async_trait;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{UserId, Interaction, Ready, Message};
use serenity::prelude::{EventHandler, Context};

use super::slash_command::{self, commands};


pub struct Handler;

struct BotInfo {
    id: UserId
}

// static SLASH_COMMANDS: OnceLock<HashMap<&str, &str>> = OnceLock::new();
static BOT: OnceLock<BotInfo> = OnceLock::new();

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        commands::record::message::send_record(&ctx, &new_message).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            slash_command::command::handle_commands(ctx, command).await;
        };
    }

    async fn ready(&self, ctx: Context, bot: Ready) {
        let bot_info = BotInfo {
            id: bot.user.id
        };
    
        let _ = BOT.set(bot_info);

        let _ = Command::set_global_application_commands(&ctx, |commands| {
            slash_command::command::regist_commands(commands)
        }).await;
    }
}
