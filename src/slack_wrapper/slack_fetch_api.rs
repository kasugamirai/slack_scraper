use super::SlackResponse;
use reqwest::Error;
use super::SlackMessage; 
use crate::slack_wrapper::slack_message_handlers::process_message;

pub struct FetchRequest {
    url: String,
    token: String,
    channel_id: String,
    limit: String,
    cursor: Option<String>,
}

impl FetchRequest {
    pub fn new(url: String, token: String, channel_id: String, limit: String, cursor: Option<String>) -> FetchRequest {
        FetchRequest {
            url,
            token,
            channel_id,
            limit,
            cursor,
        }
    }

    pub fn build(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        let mut request = client
            .get(&self.url)
            .header("Authorization", format!("Bearer {}", &self.token))
            .query(&[("channel", &self.channel_id), ("limit", &self.limit)]);

        if let Some(ref c) = self.cursor {
            request = request.query(&[("cursor", c)]);
        }

        request
    }
}

pub struct SlackFetcher {
    token: String,
    channel_id: String,
    limit: String,
}

impl SlackFetcher {
    pub fn new(token: String, channel_id: String, limit: String) -> SlackFetcher {
        SlackFetcher {
            token,
            channel_id,
            limit,
        }
    }

    pub async fn fetch_history_messages(&self) -> Result<Vec<SlackMessage>, Error> {
        let mut messages = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let request = FetchRequest::new(
                "https://slack.com/api/conversations.history".to_string(),
                self.token.clone(),
                self.channel_id.clone(),
                self.limit.clone(),
                cursor.clone(),
            ).build();

            let response = request.send().await?.json::<SlackResponse>().await?;

            if response.messages.is_empty() {
                break;
            }

            for message in &response.messages {
                println!("Message: {}", message.text);
                let _ = process_message(&message).await;
            }

            messages.extend(response.messages);

            // Update the cursor to the next value, if it exists
            cursor = response.response_metadata.next_cursor;
            if cursor.is_none() {
                break;
            }
        }

        Ok(messages)
    }
}