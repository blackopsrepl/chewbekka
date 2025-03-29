use crate::async_wrapper::chat_completion;

pub async fn debloat(content: &str) -> String {
    let prompt = format!("Read the given text and scrub it of all inclusive, woke, or corporate buzzwords—stuff like 'inclusivity,' 'stakeholder engagement,' 'building a better tomorrow,' or any other sanitized nonsense. Then, rephrase it in a stark, unfiltered, and cynically realistic way. Assume everyone involved is motivated by self-interest, power, or survival, not noble ideals. Ditch the optimism and platitudes, and tell it like it is with a sharp, no-holds-barred edge. Get to the core of what’s really being said, even if it’s ugly or inconvenient.: {}", content);
    let debloated_text = chat_completion("chewbekka", &prompt, "user").await;

    match debloated_text {
        Ok(response) => {
            println!("Raw response:\n\n {:?}", response);

            if let Some(message) = response.get("message") {
                if let Some(content) = message.get("content") {
                    return content
                        .as_str()
                        .unwrap_or("No output available")
                        .to_string();
                }
            }
            "No valid output in the response".to_string()
        }
        Err(_) => "Error occurred during debloating process".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_debloat() {
        let content = "Lorem ipsum dolor sit amet.";
        let result = debloat(content).await;
        assert_ne!(
            result, content,
            "Expected output text to be different from the original content."
        );
    }
}
