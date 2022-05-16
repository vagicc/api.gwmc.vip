use crate::db::get_connection;
use crate::schema::oauth_clients;
use crate::schema::oauth_clients::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "oauth_clients"]
pub struct OAuthClients {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub grant_types: Option<String>,
    pub scope: Option<String>,
    pub user_id: Option<i32>,
}

pub fn get_client(clientid: String) -> Option<OAuthClients> {
    let query = oauth_clients.find(clientid);

    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_client查询SQL：{:?}", sql);

    let connection = get_connection();
    let result = query.first::<OAuthClients>(&connection);

    match result {
        Ok(client) => return Some(client),
        Err(err) => {
            log::debug!("get_client查无数据：{:?}", err);
            return None;
        }
    }
}
