pub fn escape_markdown(source: &str) -> String {
    let mut result = source.to_string();
    let chars = vec![
        "*", "_", "{", "}", "[", "]", "(", ")", "#", "+", "-", ".", "!", ">", "=",
    ];
    chars.into_iter().for_each(|c| {
        let to = format!("\\{}", c);
        result = result.replace(c, to.as_str());
    });
    result
}
