use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;
use crate::models::{NewLoginActionModel, NewSignupActionModel, NewUser, User, LoginActionModel, SignupActionModel};
use crate::schema::{login_action::dsl::*, signup_action::dsl::*, users::dsl::*};

pub struct DbConnection {
    conn: PgConnection,
    login_actions: Vec<NewLoginActionModel>,
    signup_actions: Vec<NewSignupActionModel>,
    users: Vec<NewUser>,
}

impl DbConnection {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        DbConnection { conn, login_actions: Vec::new(), signup_actions: Vec::new() , users: Vec::new() }
    }

    pub fn user_exists(&mut self, public_key_param: &str) -> Result<bool, diesel::result::Error> {
        use crate::schema::users::dsl::publickey;
    
        let existing_user: Result<User, _> = users
            .filter(publickey.eq(public_key_param))
            .first(&mut self.conn);
        Ok(existing_user.is_ok())
    }
    
    pub fn insert_user(&mut self, model: NewUser) -> Result<(), diesel::result::Error> {
        self.users.push(model);
        if self.users.len() >= 1000 {
            self.flush_users()?;
        }
        Ok(())
    }

    pub fn flush_users(&mut self) -> Result<(), diesel::result::Error> {
        diesel::insert_into(users)
            .values(&self.users)
            .execute(&mut self.conn)?;
        self.users.clear();
        Ok(())
    }

    pub fn insert_login_action(&mut self, model: NewLoginActionModel) -> Result<(), diesel::result::Error> {
        self.login_actions.push(model);
        if self.login_actions.len() >= 1000 {
            self.flush_login_actions()?;
        }
        Ok(())
    }
    
    pub fn flush_login_actions(&mut self) -> Result<(), diesel::result::Error> {
        diesel::insert_into(login_action)
            .values(&self.login_actions)
            .execute(&mut self.conn)?;
        self.login_actions.clear();
        Ok(())
    }


    pub fn insert_signup_action(&mut self, model: NewSignupActionModel) -> Result<(), diesel::result::Error> {
        self.signup_actions.push(model);
        if self.signup_actions.len() >= 1000 {
            self.flush_signup_actions()?;
        }
        Ok(())
    }

    pub fn flush_signup_actions(&mut self) -> Result<(), diesel::result::Error> {
        diesel::insert_into(signup_action)
            .values(&self.signup_actions)
            .execute(&mut self.conn)?;
        self.signup_actions.clear();
        Ok(())
    }
}