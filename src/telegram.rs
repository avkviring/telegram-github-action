use telegram_bot::{Api, ChatId, ChatRef, ParseMode, SendMessage};

pub async fn send_message_to_telegram(token: String, chat_id: String, text: String) {
    let api = Api::new(token);
    let chat = ChatRef::from_chat_id(ChatId::new(chat_id.parse::<i64>().unwrap()));
    let mut message = SendMessage::new(chat, escape_markdown(text));

    message.disable_preview();
    message.parse_mode(ParseMode::Markdown);

    let response = api.send(message);
    let result = response.await;

    match result {
        Ok(result) => println!("{:?}", result),
        Err(e) => panic!("{:?}", e),
    };
}

pub fn escape_markdown(source: String) -> String {
    let mut result = source;
    let chars = vec![
        "*", "_", "{", "}", "[", "]", "(", ")", "#", "+", "-", ".", "!",
    ];
    chars.into_iter().for_each(|c| {
        let to = format!("\\{}", c);
        result = result.replace(c, to.as_str());
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::telegram::escape_markdown;

    #[test]
    fn should_parse_invalid_markdown() {
        assert_eq!(escape_markdown("hello #".to_string()), "hello \\#");
    }
}
