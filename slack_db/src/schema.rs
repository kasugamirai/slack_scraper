// @generated automatically by Diesel CLI.

diesel::table! {
    login_action (id) {
        id -> Int4,
        publickey -> Varchar,
        actiondata -> Timestamp,
    }
}

diesel::table! {
    signup_action (id) {
        id -> Int4,
        publickey -> Varchar,
        actiondata -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        publickey -> Varchar,
        createdat -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    login_action,
    signup_action,
    users,
);
