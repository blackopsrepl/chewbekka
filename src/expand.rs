use crate::async_wrapper::chat_completion;

pub async fn expand(content: &str) -> String {
    let prompt = format!("Generate a task list from this document: {}", content);
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_expand() {
        let content = "Create an AWS VPC with two EC2 webservers and an ELB.";
        let result = expand(content).await;
        assert_ne!(
            result, content,
            "Expected output text to be different from the original content."
        );
    }
}
