# slack_scraper

## Prerequisites

You need to have the following software installed to run this project:
- Rust
- Cargo
- Diesel CLI: Run `cargo install diesel_cli --no-default-features --features postgres
` to install

## Before running the project, you need to set the following environment variables in a .env file and run the Diesel migrations:

- ```cd slack_db && diesel migration run```
- SLACK_TOKEN: Your Slack token
- SLACK_CHANNEL: The Slack channel to fetch messages from
- SLACK_LIMIT: The number of messages to fetch (default is 1000)
- DATABASE_URL: The URL of your database