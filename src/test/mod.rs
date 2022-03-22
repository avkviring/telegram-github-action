use crate::event::push::process_push_event;
use crate::event::push::process_gitlab_push_event;
use crate::event::release::process_release_event;
use crate::send_message_to_telegram;
use std::fs::read_to_string;

const TOKEN: &str = "2127284370:AAHO9SCKCOsWyE7A7gOxnATk7enA8zB6-8s";
const CHAT_ID: &str = "588813729";

#[tokio::test]
async fn test_send_message() {
    send_message_to_telegram(TOKEN.to_string(), CHAT_ID.to_string(), "hello".to_string()).await;
}

#[tokio::test]
async fn test_send_push_message() {
    let message = process_push_event(read_to_string("docs/push.json").unwrap());
    send_message_to_telegram(TOKEN.to_string(), CHAT_ID.to_string(), message).await;
}

#[tokio::test]
async fn test_send_gitlab_push_message() {
    println!("{}",read_to_string("docs/push-gitlab-real.json").unwrap());
    let message = process_gitlab_push_event(read_to_string("docs/push-gitlab-real.json").unwrap());
    send_message_to_telegram(TOKEN.to_string(), CHAT_ID.to_string(), message).await;
}


#[tokio::test]
async fn test_send_release_message() {
    let message = process_release_event(read_to_string("docs/release.json").unwrap());
    println!("{}", message);
}
