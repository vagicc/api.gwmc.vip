table! {
    demo (id) {
        id -> Int4,
        name -> Varchar,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    oauth_access_tokens (access_token) {
        access_token -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
    }
}

table! {
    oauth_authorization_codes (authorization_code) {
        authorization_code -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        redirect_uri -> Nullable<Varchar>,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
        id_token -> Nullable<Varchar>,
    }
}

table! {
    oauth_clients (client_id) {
        client_id -> Bpchar,
        client_secret -> Nullable<Varchar>,
        redirect_uri -> Nullable<Varchar>,
        grant_types -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    oauth_jwt (client_id) {
        client_id -> Bpchar,
        subject -> Nullable<Varchar>,
        public_key -> Nullable<Varchar>,
    }
}

table! {
    oauth_refresh_tokens (refresh_token) {
        refresh_token -> Bpchar,
        client_id -> Bpchar,
        user_id -> Int4,
        expires -> Nullable<Timestamp>,
        scope -> Nullable<Varchar>,
    }
}

table! {
    oauth_scopes (scope) {
        scope -> Bpchar,
        is_default -> Nullable<Int2>,
    }
}

table! {
    oauth_users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        salt -> Nullable<Bpchar>,
        scope -> Nullable<Varchar>,
        create_time -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    demo,
    oauth_access_tokens,
    oauth_authorization_codes,
    oauth_clients,
    oauth_jwt,
    oauth_refresh_tokens,
    oauth_scopes,
    oauth_users,
);
