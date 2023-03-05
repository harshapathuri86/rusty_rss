use log::info;
use regex::Regex;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{
    common_types::{get_user, HandlerResult, Subscription},
    dialogue::{BotDialogue, State},
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "supported commands:")]
pub enum Command {
    #[command(description = "show this text")]
    Help,
    #[command(description = "show your subscriptions")]
    List,
    #[command(description = "delete all your subscriptions")]
    Off,
}

pub async fn handler(bot: Bot, msg: Message, dialogue: BotDialogue, cmd: Command) -> HandlerResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::List => {
            info!("Received /list form user {}", get_user(&msg));
            match dialogue.get().await? {
                Some(data) => {
                    info!("User {} subscriptions:\n{data}", get_user(&msg));
                    bot.send_message(msg.chat.id, format!("Your subscriptions:\n {data}"))
                        .await?;
                }
                None => {
                    info!("User {} subscriptions: Empty", get_user(&msg));
                    bot.send_message(msg.chat.id, "Your subscriptions list is empty")
                        .await?;
                }
            }
        }
        Command::Off => {
            dialogue.reset().await?;
            info!("Removed all subscriptions of {}", get_user(&msg));
            bot.send_message(msg.chat.id, "All your subscriptions were removed.\n Use /start to restart the bot.")
                .await?;
        }
    };
    Ok(())
}
