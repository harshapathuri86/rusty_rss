use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use teloxide::types::Message;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub feedurl: String,
    pub title: String,
    pub timestamp: DateTime<Utc>,
}

impl Subscription {
    pub fn new(feed: &str, title: &str) -> Self {
        Subscription {
            feedurl: feed.to_string(),
            title: title.to_string(),
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}",self.title , self.feedurl)
    }
}

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn get_user(msg: &Message) -> String {
    msg.chat.username().unwrap_or("<unknown>").to_string()
}
