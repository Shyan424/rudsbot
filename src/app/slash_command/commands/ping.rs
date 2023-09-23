use async_trait::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction;
use serenity::prelude::Context;

use crate::app::slash_command::command;

pub struct Ping;

static NAME: &str = "ping";

#[async_trait]
impl command::SlashCommand for Ping {

    fn get_name(&self) -> String {
        String::from(NAME)
    }


    fn regist_command(&self) -> Box<dyn FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand> {
        Box::new(|command| command.name(NAME).description("ping"))
    }

    async fn handle_command(&self, ctx : Context, command:  ApplicationCommandInteraction) {
        let response = command.create_interaction_response(&ctx, |r| {
            r.kind(interaction::InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|res_data| {
                    res_data.content("pong")
                })
        }).await;
        
        if let Err(e) = response {
            println!("Command Error {e}")
        };
    }

}

pub fn new() -> Ping {
    Ping{}
}