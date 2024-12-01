use crate::async_wrapper::chat_completion;

pub async fn summarize_content(content: &str) -> String {
    let prompt = format!("Summarize the following text: {}", content);
    let summarized_text = chat_completion("clio", &prompt, "user").await;

    match summarized_text {
        Ok(response) => {
            println!("Raw response: {:?}", response);

            if let Some(message) = response.get("message") {
                if let Some(content) = message.get("content") {
                    return content
                        .as_str()
                        .unwrap_or("No summary available")
                        .to_string();
                }
            }
            "No valid summary in the response".to_string()
        }
        Err(_) => "Error occurred during summarization".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_summarize_content() {
        let content = "Lorem ipsum dolor sit amet.";
        let result = summarize_content(content).await;
        assert_ne!(
            result, content,
            "Expected summary to be different from the original content."
        );
    }
}
