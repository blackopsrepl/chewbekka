use crate::async_wrapper::chat_completion;

pub async fn dissect_subtlety(content: &str) -> String {
    let prompt = format!("Analyze this for hidden intent and translate into the raw truth, with no diplomacy whatsoever. When you encounter inclusive language or corporate/hr lingo, analyze and reveal the underlying interests.: {}", content);
    let debloated_text = chat_completion("chewbekka", &prompt, "user").await;

    match debloated_text {
        Ok(response) => {
            println!("Raw response: {:?}", response);

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
        Err(_) => "Error occurred during subtlety dissection process".to_string(),
    }
}

pub async fn strip_jargon(content: &str) -> String {
    let prompt = format!(
        "Rewrite without all vague, corporate, or inclusive terms. State facts only: {}",
        content
    );
    let debloated_text = chat_completion("chewbekka", &prompt, "user").await;

    match debloated_text {
        Ok(response) => {
            println!("Raw response: {:?}", response);

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
        Err(_) => "Error occurred during jargon stripping process".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_dissect_subtlety() {
        let content = "Lorem ipsum dolor sit amet.";
        let result = dissect_subtlety(content).await;
        assert_ne!(
            result, content,
            "Expected output text to be different from the original content."
        );
    }

    #[tokio::test]
    async fn test_strip_jargon() {
        let content = "Lorem ipsum dolor sit amet.";
        let result = strip_jargon(content).await;
        assert_ne!(
            result, content,
            "Expected output text to be different from the original content."
        );
    }
}
