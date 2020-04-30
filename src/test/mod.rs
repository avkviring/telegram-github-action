use crate::push::process_push_event;
use crate::send_message_to_telegram;
use std::fs::read_to_string;

const TOKEN: &str = "***";
const CHAT_ID: &str = "***";

#[tokio::test]
async fn test_send_message() {
    send_message_to_telegram(TOKEN.to_string(), CHAT_ID.to_string(), "hello".to_string()).await;
}

#[tokio::test]
async fn test_send_push_message() {
    let message = process_push_event(read_to_string("docs/push.json").unwrap());
    send_message_to_telegram(TOKEN.to_string(), CHAT_ID.to_string(), message).await;
}
