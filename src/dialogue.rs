use core::fmt;

use log::*;
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::ErasedStorage, prelude::*};

use crate::common_types::{get_user, HandlerResult, Subscription};

pub type BotDialogue = Dialogue<State, ErasedStorage<State>>;
pub type DialogueStorage = std::sync::Arc<ErasedStorage<State>>;

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

    let url = user_text.trim();

    subs_list.push(Subscription::new(url.to_owned()));

    info!("Subscribed user: {} to {}", get_user(&msg), url);

    let next_message = format!("Added {} to the list of subscriptions", url);

    info!("Replied to {}: '{}'", get_user(&msg), next_message);

    bot.send_message(msg.chat.id, next_message).await?;
    dialogue.update(State::RssList(subs_list)).await?;
    Ok(())
}
