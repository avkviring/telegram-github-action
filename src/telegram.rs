use telegram_bot::{Api, ChatId, ChatRef, ParseMode, SendMessage};

pub async fn send_message_to_telegram(token: String, chat_id: String, text: String) {
    let api = Api::new(token);
    let chat = ChatRef::from_chat_id(ChatId::new(chat_id.parse::<i64>().unwrap()));
    let mut message = SendMessage::new(chat, text);

    message.disable_preview();
    message.parse_mode(ParseMode::Markdown);

    let response = api.send(message);
    let result = response.await;

    match result {
        Ok(result) => println!("{:?}", result),
        Err(e) => panic!("{:?}", e),
    };
}
