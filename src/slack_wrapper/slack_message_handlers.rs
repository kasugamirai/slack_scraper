use slack_db::db::DbConnection;
use super::{LoginAction, SignupAction, SlackMessage};
use slack_db::models::{NewLoginActionModel, NewSignupActionModel, NewUser};
use chrono::NaiveDate;

pub async fn process_message(message: &SlackMessage) -> Result<(), Box<dyn std::error::Error>> {
    let mut db_connection = DbConnection::new();
    
    if message.text.starts_with("ff-login-action:") {
        let json_start = "ff-login-action:".len();
        let json_end = message.text[json_start..].find('\n').unwrap_or_else(|| message.text.len());
        match serde_json::from_str::<LoginAction>(&message.text[json_start..json_end]) {
            Ok(login_action) => {
                let action_date = match NaiveDate::parse_from_str(&login_action.action_date, "%Y-%m-%d") {
                    Ok(date) => {
                        match date.and_hms_opt(0, 0, 0) {
                            Some(datetime) => datetime,
                            None => {
                                eprintln!("Invalid time.");
                                return Ok(());
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error parsing login action: {}", e);
                        return Ok(());
                    }
                };
                let model = NewLoginActionModel {
                    publickey: login_action.npub,
                    actiondata: action_date,
                };
                if let Ok(false) = db_connection.user_exists(&model.publickey) {
                    let new_user = NewUser {
                        publickey: model.publickey.clone(),
                        createdat: model.actiondata,
                    };
                    db_connection.insert_user(new_user)?;
                } 
                if let Ok(false) = db_connection.login_action_exists(&model.publickey, &model.actiondata) {
                    db_connection.insert_login_action(model)?;
                }
            },
            Err(_) => {
                // error parsing, skip this message
            }
        }
    } else if message.text.starts_with("ff-signup:") {
        let json_start = "ff-signup:".len();
        let json_end = message.text[json_start..].find('\n').unwrap_or_else(|| message.text.len());
        match serde_json::from_str::<SignupAction>(&message.text[json_start..json_end]) {
            Ok(signup_action) => {
                let action_date = match NaiveDate::parse_from_str(&signup_action.action_date, "%Y-%m-%d") {
                    Ok(date) => {
                        match date.and_hms_opt(0, 0, 0) {
                            Some(datetime) => datetime,
                            None => {
                                eprintln!("Invalid time.");
                                return Ok(());
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error parsing login action: {}", e);
                        return Ok(());
                    }
                };          
                let model = NewSignupActionModel {
                    publickey: signup_action.npub,
                    actiondata: action_date,
                };
                if let Ok(false) = db_connection.user_exists(&model.publickey) {
                    let new_user = NewUser {
                        publickey: model.publickey.clone(),
                        createdat: model.actiondata,
                    };
                    db_connection.insert_user(new_user)?;
                } 
                if let Ok(false) = db_connection.signup_action_exists(&model.publickey, &model.actiondata) {
                    db_connection.insert_signup_action(model)?;
                }
            },
            Err(_) => {
                // error parsing, skip this message
            }
        }
    }

    Ok(())
}