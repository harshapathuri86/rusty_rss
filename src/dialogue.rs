use core::fmt;

use log::*;
use serde::{Deserialize, Serialize};
use teloxide::{dispatching::dialogue::ErasedStorage, prelude::*};

use crate::common_types::{HandlerResult, Subscription};

pub type BotDialogue = Dialogue<State, ErasedStorage<State>>;
pub type DialogueStorage = std::sync::Arc<ErasedStorage<State>>;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]

pub enum State {
    #[default]
    Start,
    RssList {
        messages: Vec<Subscription>,
    },
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Start => write!(f, "State: Start"),
            State::RssList { messages } => write!(f, "{:?}", messages),
        }
    }
}

pub async fn start(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
) -> HandlerResult {
    info!("Starting chat: in dialogue");

    bot.send_message(msg.chat.id, "Hi am a rusty rss bot!").await?;
    dialogue
        .update(State::RssList {
            messages: vec![]
        })
        .await?;
    Ok(())
}

pub async fn rss_list(
    bot: Bot,
    dialogue: BotDialogue,
    messages: Vec<Subscription>,
    msg: Message,
) -> HandlerResult {
    let mut next_messages = messages.clone();
    let Some(user_text) = msg.text() else {
        warn!("Received irregular message: {:?}", msg);
        return Ok(())
    };

    info!(
        "Received message from {}: '{}'",
        msg.chat.username().unwrap_or("<unknown>"),
        user_text
    );
    next_messages.push(Subscription::new(user_text.trim_start().trim_end().to_owned()));

    let next_message = Subscription::new("dummy next message".to_string());

    info!(
        "Replied to {}: '{}'",
        msg.chat.username().unwrap_or("<unknown>"),
        next_message.feed
    );

    bot.send_message(msg.chat.id, next_message.feed).await?;
    dialogue
        .update(State::RssList {
            messages: next_messages,
        })
        .await?;
    Ok(())
}
