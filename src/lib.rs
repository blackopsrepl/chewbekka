pub mod async_wrapper;
pub mod extract;

use std::collections::HashMap;

use async_wrapper::chat_completion;

pub async fn write_md_file(output_files: &HashMap<String, String>, summmarize: bool) {
    let concatenated_output: String = output_files
        .values()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n\n");

    // TODO: pass filename as argument
    let output_file = "output.md";
    
    // summarization across the entire array vs simple concatenation
    if summmarize {
        let output = process_content(&concatenated_output, "summarize").await;
        std::fs::write(output_file, output).unwrap();
    } else {
        std::fs::write(output_file, &concatenated_output).unwrap();
    }
}

pub async fn process_content(content: &str, task: &str) -> String {
    let debloat = format!("Read the given text and scrub it of all inclusive, woke, or corporate buzzwords—stuff like 'inclusivity,' 'stakeholder engagement,' 'building a better tomorrow,' or any other sanitized nonsense. Then, rephrase it in a stark, unfiltered, and cynically realistic way. Assume everyone involved is motivated by self-interest, power, or survival, not noble ideals. Ditch the optimism and platitudes, and tell it like it is with a sharp, no-holds-barred edge. Get to the core of what’s really being said, even if it’s ugly or inconvenient.: {}", content);
    let expand = format!("Generate a task list from this document: {}", content);
    let summarize = format!("Summarize the following text: {}", content);

    let prompt = match task {
        "debloat" => debloat,
        "expand" => expand,
        "summarize" => summarize,
        _ => "Invalid task".to_string(),
    };

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
    use tokio;

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