use crate::db::get_connection;
use crate::schema::oauth_access_tokens;
use crate::schema::oauth_access_tokens::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体合二为一(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "oauth_access_tokens"]
pub struct OAuthAccessTokens {
    pub access_token: String,
    pub client_id: String,
    pub user_id: i32,
    pub expires: Option<NaiveDateTime>,
    pub scope: Option<String>,
}
impl OAuthAccessTokens {
    pub fn insert(&mut self) -> Option<OAuthAccessTokens> {
        /* 处理expires有效时间 */
        if self.expires.is_none() {
            let now_date_time=crate::common::now_naive_date_time();
            self.expires = Some(now_date_time);
        }

        let connection = get_connection();
        let insert_data = diesel::insert_into(oauth_access_tokens)
            .values(self.clone())
            .get_result::<OAuthAccessTokens>(&connection); //这里还要处理，不能中断程序

        match insert_data {
            Ok(data) => {
                log::debug!("表(oauth_access_tokens)成功插入数据：{:?}", data);
                return Some(data);
            }
            Err(database_error) => {
                // println!("插入出错: {:#?}", database_error); // diesel::result::Error::DatabaseError;
                // crate::common::type_v(database_error);  //查看所属性形
                /*
                Err(
                DatabaseError(
                    UniqueViolation,
                    "duplicate key value violates unique constraint \"oauth_access_tokens_pkey\"",
                )
                */
                match database_error {
                    diesel::result::Error::DatabaseError(error_kind, message) => {
                        log::debug!("表(oauth_access_tokens)插入错误：{:#?}", error_kind); //2022-05-10 14:20:53 [DEBUG] - 数据插入错误：UniqueViolation
                        log::debug!("表(oauth_access_tokens)插入错误提示：{:#?}", message);
                        //2022-05-10 14:20:53 [DEBUG] - 数据插入错误提示："duplicate key value violates unique constraint \"oauth_access_tokens_pkey\""
                    }
                    _ => {
                        log::error!(
                            "表(oauth_access_tokens)插入其它类型错误：{:#?}",
                            database_error
                        );
                    }
                }
                return None;
            }
        }
    }
}

pub fn get_user_id(token: &str) -> Option<OAuthAccessTokens> {
    let query = oauth_access_tokens.find(token);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("oauth_access_tokens查询（get_user_id）SQL：{:?}", sql);

    let connection = get_connection();
    let result = query.first::<OAuthAccessTokens>(&connection);

    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("查无数据：{}", e);
            return None;
        }
    }
}
