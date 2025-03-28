use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a message structure used in the chat request.
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// Represents a request to perform a chat completion.
#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    /// Name or identifier of the model to use for chat completion
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

/// Represents a request to pull a model.
#[derive(Debug, Serialize, Deserialize)]
struct PullRequest {
    name: String,
    stream: bool,
}

/// Represents a request to generate embeddings.
#[derive(Debug, Serialize, Deserialize)]
struct EmbRequest {
    model: String,
    prompt: String,
}

/// Represents a request to push a model.
#[derive(Debug, Serialize, Deserialize)]
struct PushRequest {
    name: String,
    stream: bool,
}

/*
Performs a chat completion request.
Sends a chat completion request using the specified model, message content, and role.
Returns JSON payload.
*/

pub async fn chat_completion(
    model: &str,
    content: &str,
    role: &str,
) -> Result<Value, reqwest::Error> {
    let req = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": role,
                "content": content
            }
        ],
        "stream": false
    });

    // Send the POST request and get the response as JSON
    let response = Client::new()
        .post("http://localhost:11434/api/chat")
        .json(&req)
        .send()
        .await?;

    let response_json = response.json::<Value>().await?;

    Ok(response_json)
}

/*
Performs a model pull request.
Sends a request to pull a model identified by `name`.
Uses streaming mode if `stream_mode` is true.
Returns an error if the request fails.
*/
#[tokio::main]
pub async fn pull_model(name: &str, stream_mode: bool) -> Result<(), reqwest::Error> {
    let req = PullRequest {
        name: String::from(name),
        stream: stream_mode,
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/pull")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}", response_json);
    Ok(())
}

/*
Performs a request to generate embeddings.
Sends a request to generate embeddings using the specified model and prompt.
Returns an error if the request fails.
*/
#[tokio::main]
pub async fn gen_embeddings(model: &str, prompt: &str) -> Result<(), reqwest::Error> {
    let req = EmbRequest {
        model: String::from(model),
        prompt: String::from(prompt),
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/embeddings")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}", response_json);
    Ok(())
}
/*
Lists available models.
Retrieves a list of available models from the server.
Prints the response to standard output.
Returns an error if the request fails.
*/
#[tokio::main]
pub async fn list_models() -> Result<(), reqwest::Error> {
    let response = reqwest::get("http://localhost:11434/api/tags").await?;
    println!("{:#?}", response);
    Ok(())
}

/*
Performs a model push request.
Sends a request to push a model identified by `name`.
Uses streaming mode if `stream_mode` is true.
Returns an error if the request fails.
*/
#[tokio::main]
pub async fn push_models(name: &str, stream_mode: bool) -> Result<(), reqwest::Error> {
    let req = PushRequest {
        name: String::from(name),
        stream: stream_mode,
    };
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/push")
        .json(&req)
        .send()
        .await?;
    let response_json: serde_json::Value = response.json().await?;
    println!("{:#?}", response_json);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_chat_completion() {
        std::mem::drop(chat_completion("model_name", "Hello!", "user"));
    }

    #[test]
    fn test_pull_model() {
        let _ = pull_model("model_name", false);
    }

    #[test]
    fn test_gen_embeddings() {
        let _ = gen_embeddings("model_name", "Generate embeddings from this prompt");
    }

    #[test]
    fn test_list_models() {
        let _ = list_models();
    }

    #[test]
    fn test_push_models() {
        let _ = push_models("model_name", true);
    }
}
