CREATE TABLE login_action (
    id SERIAL PRIMARY KEY,
    publicKey VARCHAR REFERENCES users(publicKey) UNIQUE NOT NULL,
    actionData TIMESTAMP NOT NULL
);

CREATE INDEX idx_login_action_actionData ON login_action(actionData);