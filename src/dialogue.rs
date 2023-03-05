use core::fmt;

use log::*;
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::ErasedStorage, prelude::*};

use crate::common_types::{get_user, HandlerResult, Subscription};

pub type BotDialogue = Dialogue<State, ErasedStorage<State>>;
pub type DialogueStorage = std::sync::Arc<ErasedStorage<State>>;

use crate::validate;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]

pub enum State {
    #[default]
    Start,
    RssList(Vec<Subscription>),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Start => write!(f, ""),
            State::RssList(subs) => {
                for sub in subs {
                    write!(f, "{}\n", sub)?;
                }
                Ok(())
            }
        }
    }
}

pub async fn start(bot: Bot, dialogue: BotDialogue, msg: Message) -> HandlerResult {
    info!("Starting chat: in dialogue");

    bot.send_message(msg.chat.id, "Hi am a rusty rss bot!")
        .await?;
    dialogue.update(State::RssList(vec![])).await?;
    Ok(())
}

pub async fn rss_list(
    bot: Bot,
    dialogue: BotDialogue,
    subs: Vec<Subscription>,
    msg: Message,
) -> HandlerResult {
    let mut subs_list = subs.clone();
    let Some(user_text) = msg.text() else {
        warn!("Received irregular message: {:?}", msg);
        return Ok(())
    };

    info!("Received message from {}: '{}'", get_user(&msg), user_text);
    
    let new_subscription: Subscription;
    let url = user_text.trim();
    match validate::parse_feed(url).await {
        Ok(feed_source) => {
            match feed_source {
                validate::Feed::Atom(feed) => {
                    new_subscription = Subscription::new(url, &feed.title());
                } 
                validate::Feed::RSS(channel) => {
                    new_subscription = Subscription::new(url, &channel.title());
                },
            }
        }
        Err(_) => {
            // bot.send_message(msg.chat.id, format!("Error validating given url:{}", e ))
            bot.send_message(msg.chat.id, format!("This does not look like a valid Atom/RSS feed.\nTry something like https://xkcd.com/rss.xml"))
            .await?;
            return Ok(());
        },
    }

    let title = new_subscription.title.to_string();

    subs_list.push(new_subscription);

    info!("Subscribed user: {} to {}", get_user(&msg), url);

    let mut message = "Subscribed to ".to_owned();
    message.push_str(&teloxide::utils::markdown::link(url,&title));

    bot.send_message(msg.chat.id,message).await?;

    info!("Replied to {}: '{}'", get_user(&msg), format!("Subscribed to {}", url));


    dialogue.update(State::RssList(subs_list)).await?;
    Ok(())
}
