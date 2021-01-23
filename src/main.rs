#[macro_use]
extern crate serde_derive;

use std::env;

use crate::event::push::process_push_event;
use crate::event::release::process_release_event;
use crate::telegram::send_message_to_telegram;

pub mod event;
pub mod markdown;
pub mod telegram;

#[cfg(test)]
pub mod test;

#[tokio::main]
async fn main() {
    let event = env::var("event").expect("Add environment event:${{ github.event }}");
    let message = if event.contains("\"release\"") {
        process_release_event(event.to_string())
    } else {
        process_push_event(event.to_string())
    };

    let token = env::var("telegram_token")
        .expect("Add environment telegram_token:${{ secrets.telegram_token }}");
    let chat_id =
        env::var("telegram_to").expect("Add environment telegram_to:${{ secrets.telegram_to }}");

    send_message_to_telegram(token, chat_id, message).await;
}
