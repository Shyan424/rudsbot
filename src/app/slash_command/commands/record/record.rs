use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::{ChannelId, InteractionResponseType};
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, CommandDataOptionValue, CommandDataOption};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;

use crate::app::slash_command::command::SlashCommand;

use super::message;


pub struct Record {}

static NAME: &str = "record";

impl Record {
    pub fn new() -> Record {
        Record {}
    }
}

#[async_trait]
impl SlashCommand for Record {
    fn get_name(&self) -> String {
        String::from(NAME)
    }

    fn regist_command(&self) -> Box<dyn FnOnce(&mut CreateApplicationCommand) ->  &mut CreateApplicationCommand>  {
        Box::new(
            |c| {
                c.name(NAME).description("record channel message")
                    .create_option(|option| {
                        option
                            .name("action")
                            .description("add: add record channel\ndel: delete record channel\nshow: show now record channel")
                            .required(true)
                            .kind(CommandOptionType::String)
                            .add_string_choice("add", "add")
                            .add_string_choice("del", "del")
                            .add_string_choice("show", "show")
                    })
                    .create_option(|option| {
                        option
                            .name("channel_id")
                            .description("add or del channel id")
                            .kind(CommandOptionType::String)
                    })
            })
    }


    async fn handle_command(&self, ctx: Context, command: ApplicationCommandInteraction) {
        let options = &command.data.options;
        let this_channdel_id = command.channel_id;
        // let c = command.guild_id.unwrap().channels(&ctx).await;
        
        let res_str = match handle_options(options) {
            Ok(Action::Add(id)) => handel_add(&ctx, this_channdel_id, ChannelId(id)).await,
            Ok(Action::Del(id)) => handel_del(this_channdel_id, ChannelId(id)),
            Ok(Action::Show()) => handel_show(this_channdel_id),
            Err(e) => format!("something wrong {e}")
        };
        
        let _ = command.create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| data.content(res_str))
        }).await;
    }
}

enum Action {
    Add(u64),
    Del(u64),
    Show(),
}

fn handle_options(options: &Vec<CommandDataOption>) -> Result<Action, String> {
    let option_len = options.len();

    let mut option_action = String::new();
    let mut option_channelid = 0;

    for option in options {
        match option.name.as_str() {
            "action" => {
                if let Some(CommandDataOptionValue::String(s)) = &option.resolved {
                    option_action.push_str(s);
                }
            },
            "channel_id" => {
                if let Some(CommandDataOptionValue::String(s)) = &option.resolved {
                    match s.parse::<u64>() {
                        Ok(u) => option_channelid = u,
                        Err(e) => return Err(e.to_string()),
                    }
                }
            },
            _ => ()
        };
    }

    if (option_len < 2 && option_action != "show")
        || (option_action != "show" && option_channelid == 0) {
        return Err(String::from("error options"));
    }

    match option_action.as_str() {
        "add" => Ok(Action::Add(option_channelid)),
        "del" => Ok(Action::Del(option_channelid)),
        "show" => Ok(Action::Show()),
        _ => Err(String::from("error options"))
    }
}

async fn handel_add(ctx : &Context, this_channel: ChannelId, record_channel: ChannelId) -> String {
    let this_cname = this_channel.name(ctx).await;
    
    let is_channel_ok = match this_cname {
        Some(name) => {
            match record_channel.say(ctx, format!("你被{name}監控了")).await {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        None => false,
    };

    if !is_channel_ok {
        return String::from("where is channel")
    }

    {
        let map = Arc::clone(message::record_channel());
        let mut map = map.lock().await;

        match map.get_mut(&record_channel) {
            Some(set) => {
                set.insert(this_channel);
            },
            None => {
                let mut set = HashSet::new();
                set.insert(this_channel);
                map.insert(record_channel, set);
            }
        };
    }

    String::from("OK")
}

fn handel_del(this_channel: ChannelId, record_channel: ChannelId) -> String {
    String::from("還沒做好...")
}

fn handel_show(this_channel: ChannelId) -> String {
    String::from("還沒做好...")
}
