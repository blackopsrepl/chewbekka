pub mod async_wrapper;
pub mod extract;
pub mod prompts;

use std::{collections::HashMap, path::PathBuf};

use async_wrapper::chat_completion;
use prompts::get_prompt;

pub async fn write_md_file(
    input_files: &HashMap<String, String>,
    output_file: &PathBuf,
    summmarize: bool,
) {
    let concatenated_output: String = input_files
        .values()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n\n");

    // summarization across the entire array vs simple concatenation
    if summmarize {
        let output = process_content(&concatenated_output, "summarize").await;
        std::fs::write(output_file, output).unwrap();
    } else {
        std::fs::write(output_file, &concatenated_output).unwrap();
    }
}

pub async fn process_content(content: &str, task: &str) -> String {
    let prompt = get_prompt(task, content);

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
        Err(_) => "Error occurred during process".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_summarize_content() {
        let content = "Lorem ipsum dolor sit amet.";
        let result = process_content(content, "summarize").await;
        println!("\n\nResult:\n\n {:?}", result);
        assert_ne!(
            result, content,
            "Expected summary to be different from the original content."
        );
    }

    #[tokio::test]
    async fn test_debloat_content() {
        let content = "Lorem ipsum dolor sit amet.";
        let result = process_content(content, "debloat").await;
        println!("\n\nResult:\n\n {:?}", result);
        assert_ne!(
            result, content,
            "Expected summary to be different from the original content."
        );
    }

    #[tokio::test]
    async fn test_expand_content() {
        let content = "Create an AWS VPC with two redundant EC2 webservers, a postgresql backend, a backend server and an ELB.";
        let result = process_content(content, "expand").await;
        println!("\n\nResult:\n\n {:?}", result);
        assert_ne!(
            result, content,
            "Expected summary to be different from the original content."
        );
    }
}
