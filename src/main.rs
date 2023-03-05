use dialogue::DialogueStorage;
use dotenv::dotenv;
use log::warn;
use teloxide::{
    dispatching::dialogue::{serializer::Json, ErasedStorage, SqliteStorage, Storage},
    prelude::*,
};

mod common_types;

use crate::dialogue::State;

mod commands;
mod dialogue;
mod validate;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let telegram_api_token =
        std::env::var("TELEGRAM_API_TOKEN").expect("TELEGRAM_API_TOKEN must be set");

    let db_path = std::env::var("DB_PATH").expect("DB_PATH must be set");

    let storage: DialogueStorage = SqliteStorage::open(&db_path, Json).await.unwrap().erase();

    let bot = Bot::new(telegram_api_token);

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, ErasedStorage<dialogue::State>, dialogue::State>()
            .branch(
                dptree::entry()
                    .filter_command::<commands::Command>()
                    .endpoint(commands::handler),
            )
            .branch(
                dptree::entry()
                    .branch(dptree::case![State::Start].endpoint(dialogue::start))
                    .branch(dptree::case![State::RssList(subs)].endpoint(dialogue::rss_list)),
            ),
    )
    .dependencies(dptree::deps![storage])
    .default_handler(|upd| async move {
        warn!("Unhandled update: {:?}", upd);
    })
    .error_handler(LoggingErrorHandler::with_custom_text(
        "An error has occurred in the dispatcher",
    ))
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
