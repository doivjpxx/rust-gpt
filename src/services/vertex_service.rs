use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GeminiContent {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    pub contents: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    #[serde(rename = "content")]
    pub content: Content,
    #[serde(rename = "finishReason")]
    finish_reason: String,
    #[serde(rename = "index")]
    index: usize,
    #[serde(rename = "safetyRatings")]
    safety_ratings: Vec<SafetyRating>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
    #[serde(rename = "role")]
    role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SafetyRating {
    #[serde(rename = "category")]
    category: String,
    #[serde(rename = "probability")]
    probability: String,
}

pub async fn get_gpt_message(message: &str) -> Result<ApiResponse, String> {
    let api_key = std::env::var("GEMINI_KEY").expect("GEMINI_KEY must be set");
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let body: GeminiRequest = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: message.to_owned() }],
            role: None,
        }],
    };

    let response = client.post(url).json(&body).send().await;

    match response {
        Ok(response) => {
            let json = response.json::<ApiResponse>().await.unwrap();
            Ok(json)
        }
        Err(e) => Err(e.to_string()),
    }
}
