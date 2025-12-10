use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub fn send_request(messages: &[Message]) -> Result<String, String> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY environment variable not set. Please set it to use the AI feature.".to_string())?;
    let url = "https://api.openai.com/v1/chat/completions";

    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": messages,
    });

    // ureq 3.x usage
    let response = ureq::post(url)
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .send_json(body)
        .map_err(|e| format!("Request failed: {}", e))?;

    let json: serde_json::Value = response.into_body().read_json().map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No content in response".to_string())
}
