const DEBLOAT_PROMPT: &str = "Read the given text and scrub it of all inclusive, woke, or corporate buzzwords—stuff like 'inclusivity, ' 'stakeholder engagement, ' 'building a better tomorrow, ' or any other sanitized nonsense. Then, rephrase it in a stark, unfiltered, and cynically realistic way. Assume everyone involved is motivated by self-interest, power, or survival, not noble ideals. Ditch the optimism and platitudes, and tell it like it is with a sharp, no-holds-barred edge. Get to the core of what’s really being said, even if it’s ugly or inconvenient: ";
const EXPAND_PROMPT: &str = "Generate a task list from this document: ";
const SUMMARIZE_PROMPT: &str = "Summarize the following text: ";
const INVALID_TASK: &str = "Invalid task";

pub fn get_prompt(task: &str, content: &str) -> String {
    match task {
        "debloat" => format!("{}{}", DEBLOAT_PROMPT, content),
        "expand" => format!("{}{}", EXPAND_PROMPT, content),
        "summarize" => format!("{}{}", SUMMARIZE_PROMPT, content),
        _ => INVALID_TASK.to_string(),
    }
}

#[tokio::test]
async fn test_get_prompt() {
    let content = "This is a test content, from a diverse, inclusive and demure source. We are sorry, but we decided to proceed with another candidate that is the perfect culture fit.";
    
    let prompt = get_prompt("debloat", content);
    assert_eq!(prompt, format!("Read the given text and scrub it of all inclusive, woke, or corporate buzzwords—stuff like 'inclusivity, ' 'stakeholder engagement, ' 'building a better tomorrow, ' or any other sanitized nonsense. Then, rephrase it in a stark, unfiltered, and cynically realistic way. Assume everyone involved is motivated by self-interest, power, or survival, not noble ideals. Ditch the optimism and platitudes, and tell it like it is with a sharp, no-holds-barred edge. Get to the core of what’s really being said, even if it’s ugly or inconvenient: {}", content));
    
    let prompt = get_prompt("expand", content);
    assert_eq!(prompt, format!("Generate a task list from this document: {}", content));
    
    let prompt = get_prompt("summarize", content);
    assert_eq!(prompt, format!("Summarize the following text: {}", content));
}
