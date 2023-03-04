use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub feed: String,
    pub timestamp: DateTime<Utc>,
}

impl Subscription {
    pub fn new(feed: String) -> Self {
        Subscription {
            feed,
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.feed)
    }
}

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
