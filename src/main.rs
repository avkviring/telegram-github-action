#[macro_use]
extern crate serde_derive;

use std::panic::resume_unwind;
use std::{env, fs};

use telegram_bot::*;
use telegram_bot::{
    Api, ChatId, ChatRef, Error, GetMe, Message, MessageOrChannelPost, Request, ResponseType,
    SendMessage, UpdateKind,
};

use crate::push::process_push_event;

pub mod github;
pub mod push;

#[tokio::main]
async fn main() {
    println!("start actions");
    let event = env::var("event").expect("Add environment event:${{ github.event }}");
    let message = process_push_event(event.to_string());
    send_message(message).await;
}

// "1064561948:AAGSBEsVJbyKTFtSmYaTkYWDkFvEdgYBtRA".to_string();
// 343874845
async fn send_message(text: String) {
    let token = env::var("telegram_token")
        .expect("Add environment telegram_token:${{ secrets.telegram_token }}");
    let chat_id =
        env::var("telegram_to").expect("Add environment telegram_to:${{ secrets.telegram_to }}");

    let api = Api::new(token);

    let chat = ChatRef::from_chat_id(ChatId::new(chat_id.parse::<i64>().unwrap()));
    let mut message = SendMessage::new(chat, text);

    message.disable_preview();
    message.parse_mode(ParseMode::Markdown);

    let response = api.send(message);
    let result = response.await;

    match result {
        Ok(result) => println!("{:?}", result),
        Err(e) => panic!("{}", e),
    };
}
