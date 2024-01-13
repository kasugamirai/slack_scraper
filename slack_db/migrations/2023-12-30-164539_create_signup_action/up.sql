-- Your SQL goes here
CREATE TABLE signup_action (
    id SERIAL PRIMARY KEY,
    publicKey VARCHAR REFERENCES users(publicKey) UNIQUE NOT NULL,
    actionData TIMESTAMP NOT NULL
);

CREATE INDEX idx_signup_action_actionData ON signup_action(actionData)