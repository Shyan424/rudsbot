use serde_yaml::Value;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::{group, command};
use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::model::prelude::Message;
use serenity::prelude::{GatewayIntents, Context};

use super::event_handle;

pub async fn start() {
    let config: Value = serde_yaml::from_reader(std::fs::File::open("config.yaml").expect("config open fail")).expect("load config fail");
    let token = config.get("token").expect("no token").as_str().unwrap();

    let intents = GatewayIntents::non_privileged()
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::GUILD_MESSAGES;

    let framework = StandardFramework::new()
    .configure(|c| c.prefix("~"))
    .group(&NORMALMESSAGE_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(event_handle::Handler)
        .framework(framework)
        .await
        .expect("client build fail");

        if let Err(e) = client.start().await {
            println!("start error {}", e);
        }
}

#[group]
#[commands(ping)]
struct NormalMessage;

#[command]
// 設定時設定成 .configure(|c| c.prefix("~"))  會依 fn 名稱呼叫 (~ping)
// arg 會回後面的參數 ex: ~ping cc 就會給 cc
async fn ping(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let author_nick_name = match msg.author_nick(context).await {
        Some(name) => name.clone(),
        None => msg.author.name.clone(),
    };

    let mut arg_vet: Vec<String> = Vec::new();
    for a in args.iter::<String>() {
        arg_vet.push(a.unwrap_or(String::from("")));
    }

    if let Err(e) = msg.channel_id.say(context, format!("pong啦 user: {} args: {}", author_nick_name, arg_vet.join(" ??? "))).await {
        println!("ping error {}", e);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use serde_yaml::Value;

    #[test]
    #[ignore]
    fn load_config_test() {
        let config: Value = serde_yaml::from_reader(std::fs::File::open("./config_example.yaml").expect("open fail")).expect("reader fail");
        let test = config.get("test");
    
        println!("{}", test.expect("test none").get("token").expect("token none").as_str().expect("token fail"));   
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Test {
        token: String,
        test: Token,
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Token {
        name: String,
        token: String,
    }

    #[test]
    #[ignore]
    fn load_struct_config_test() {
        let config: Test = serde_yaml::from_reader(std::fs::File::open("./config_example.yaml").expect("open fail")).expect("reader fail");
    
        println!("{:#?}", config);
    }

}