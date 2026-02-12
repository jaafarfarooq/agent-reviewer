/// Lightweight LLM client wrapper.
///
/// Responsibilities:
/// - Wrap HTTP calls to the LLM provider
/// - Authenticate with an API key
/// - Send prompts and return assistant content as cleaned strings
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::json;

#[derive(Clone)]
pub struct LlmClient {
    http: Client,
    api_key: String,
}

impl LlmClient {
    pub fn new(api_key: String) -> Self {
        Self {
            http: Client::new(),
            api_key,
        }
    }

    fn sanitize_content(content: &str) -> String {
    content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
        .to_string()
    }


    pub async fn send_prompt(&self, prompt: &str) -> Result<String> {
        let response = self.http
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&self.api_key)
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": [
                { "role": "system", "content": "You are a senior software engineer." },
                { "role": "user", "content": prompt }
            ],
            "temperature": 0.2
        }))
        .send()
        .await?;

        let status = response.status();
        let raw_text = response.text().await?;

        //println!("LLM HTTP status: {}", status);
        //println!("LLM raw response:\n{}", raw_text);

        let json: serde_json::Value = serde_json::from_str(&raw_text)?;

        // Extract the assistant content from the response JSON and return it
        let content = json
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c0| c0.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Invalid LLM response: missing choices[0].message.content"))?;

        let cleaned = Self::sanitize_content(content);
        Ok(cleaned)

    }
}
