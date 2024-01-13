use dotenvy::dotenv;
mod slack_wrapper;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let token = std::env::var("SLACK_TOKEN").expect("SLACK_TOKEN must be set");
    let channel = std::env::var("SLACK_CHANNEL").expect("SLACK_CHANNEL must be set");
    let limit = std::env::var("SLACK_LIMIT").unwrap_or_else(|_| "1000".to_string());

    let fetcher = slack_wrapper::slack_fetch_api::SlackFetcher::new(token, channel, limit);
    fetcher.fetch_history_messages().await.map(|_| ())
}
