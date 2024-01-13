use serde::Deserialize;

#[derive(Deserialize)]
pub struct SlackResponse {
    pub messages: Vec<SlackMessage>,
    pub response_metadata: ResponseMetadata,
}

#[derive(Deserialize)]
pub struct ResponseMetadata {
    pub next_cursor: Option<String>,
}
#[derive(Deserialize)]
pub struct SlackMessage {
    pub text: String,
}

#[derive(Deserialize)]
struct LoginAction {
    #[serde(rename = "actionDate")]
    action_date: String,
    #[serde(rename = "npub")]
    npub: String,
}

#[derive(Deserialize)]
struct SignupAction {
    #[serde(rename = "nPubkey")]
    npub: String,
    #[serde(rename = "actionDate")]
    action_date: String,
}


pub mod slack_fetch_api;
pub mod slack_message_handlers;
