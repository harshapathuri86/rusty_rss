use regex::Regex;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{
    common_types::{HandlerResult, Subscription},
    dialogue::{BotDialogue, State},
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "supported commands:"
)]
pub enum Command {
    #[command(description = "show this text")]
    Help,
    #[command(description = "show your subscriptions")]
    List,
    #[command(
        description = "delete all your subscriptions"
    )]
    Off,
}

pub async fn handler(bot: Bot, msg: Message, dialogue: BotDialogue, cmd: Command) -> HandlerResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::List => {
            match dialogue.get().await? {
                Some(data) => {
                    println!("current_state: {:#?}", data);
                    bot.send_message(msg.chat.id, format!("Your subscriptions:\n {data}")).await?;
                }
                None => {
                    println!("current_state: Empty" );
                    bot.send_message(msg.chat.id, "Your subscriptions list is empty").await?;
                }
            }
        }
        Command::Off => {
            dialogue.reset().await?;
            bot.send_message(msg.chat.id, "All your subscriptions were removed").await?;
        }
    };
    Ok(())
}
