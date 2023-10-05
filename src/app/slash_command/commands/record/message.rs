use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};

use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use tokio::sync::Mutex;

pub async fn send_record(ctx: &Context, message: &Message) {
    let map = Arc::clone(record_channel());
    let map = map.lock().await;

    if let Some(channels) = map.get(&message.channel_id) {
        for c in channels {
            let msg = format!("{}: {}", &message.author.name, &message.content);
            let _ = c.say(ctx, msg).await;
        }
    }
}

type RecordMap = Arc<Mutex<HashMap<ChannelId, HashSet<ChannelId>>>>;

pub fn record_channel() -> &'static RecordMap {
    static RECORD_CHANNEL_MAP: OnceLock<RecordMap> = OnceLock::new();
    RECORD_CHANNEL_MAP.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}