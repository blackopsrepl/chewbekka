pub mod async_wrapper;
pub mod extract;
pub mod prompts;

use std::{collections::HashMap, path::PathBuf};
use std::sync::Mutex;
use async_wrapper::chat_completion;
use prompts::get_prompt;

pub async fn write_md_file(input_files: &HashMap<String, String>, output_file: &PathBuf) {
    let concatenated_output = concatenate_files(input_files);
    std::fs::write(output_file, &concatenated_output).unwrap();
}

pub async fn pre_tasks(
    input_files: Mutex<HashMap<String, String>>,
    task: &str,
) -> Mutex<HashMap<String, String>> {
    match task {
        "docugen" => {
            let input_files = concatenate_files(&input_files.lock().unwrap().clone());
            let output_files = Mutex::new(HashMap::new());
            output_files
                .lock()
                .unwrap()
                .insert("concatenated".to_string(), input_files);
            output_files
        }
        _ => {
            input_files
        }
    }
}

pub async fn post_tasks(
    input_files: Mutex<HashMap<String, String>>,
    task: &str,
) -> Mutex<HashMap<String, String>> {
    match task {
        "summarize" => {
            let output_files = Mutex::new(HashMap::new());
            let summary = &input_files.lock().unwrap().clone();
            let summary = summarize_files_across(summary).await;
            output_files.lock().unwrap().insert("summary".to_string(), summary);
            output_files
        }
        _ => {
            input_files
        }
    }
}

async fn summarize_files_across(input_files: &HashMap<String, String>) -> String {
    return process_content(&concatenate_files(input_files), "summarize").await;
}

fn concatenate_files(input_files: &HashMap<String, String>) -> String {
    input_files
        .values()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n\n")
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
        
        Err(e) => "Error occurred during process\n\n".to_string() + &e.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_summarize_content() {
        let content = "This is a test content, from a diverse, inclusive and demure source. We are sorry, but we decided to proceed with another candidate that is the perfect culture fit.";
        let result = process_content(content, "summarize").await;
        println!("\n\nResult:\n\n {:?}", result);
        assert_ne!(
            result, content,
            "Expected summary to be different from the original content."
        );
    }

    #[tokio::test]
    async fn test_debloat_content() {
        let content = "This is a test content, from a diverse, inclusive and demure source. We are sorry, but we decided to proceed with another candidate that is the perfect culture fit.";
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

    #[tokio::test]
    async fn test_docugen_content() {
        let content = std::fs::read_to_string("src/main.rs").unwrap();
        let result = process_content(&content, "docugen").await;
        println!("\n\nResult:\n\n {:?}", result);
        assert_ne!(
            result, content,
            "Expected summary to be different from the original content."
        );
    }
}
