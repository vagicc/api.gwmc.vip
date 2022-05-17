use crate::db::get_connection;
use crate::schema::oauth_refresh_tokens;
use crate::schema::oauth_refresh_tokens::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体合二为一(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "oauth_refresh_tokens"]
pub struct OAuthRefreshTokens {
    pub refresh_token: String,
    pub client_id: String,
    pub user_id: i32,
    pub expires: Option<NaiveDateTime>,
    pub scope: Option<String>,
}
impl OAuthRefreshTokens {
    pub fn insert(&mut self) -> Option<OAuthRefreshTokens> {
        if self.expires.is_none() {
            // use chrono::prelude::{Local, NaiveDate, NaiveDateTime};
            let fmt = "%Y-%m-%d %H:%M:%S";
            let now = chrono::prelude::Local::now();
            let dft = now.format(fmt);
            let str_date = dft.to_string();
            // println!("当前时间：{}", str_date);
            let now_date_time =
                chrono::prelude::NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();
            // let now_date = chrono::prelude::NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d").unwrap();

            self.expires = Some(now_date_time);
        }

        let connection = get_connection();
        let insert_data = diesel::insert_into(oauth_refresh_tokens)
            .values(self.clone())
            .get_result::<OAuthRefreshTokens>(&connection);

        match insert_data {
            Ok(data) => {
                log::debug!("表(oauth_refresh_tokens)成功插入数据：{:?}", data);
                return Some(data);
            }
            Err(error) => {
                match error {
                    diesel::result::Error::DatabaseError(error_kind, message) => {
                        log::debug!("表(oauth_refresh_tokens)插入错误：{:#?}", error_kind); //2022-05-10 14:20:53 [DEBUG] - 数据插入错误：UniqueViolation
                        log::debug!("表(oauth_refresh_tokens)插入错误提示：{:#?}", message);
                        //2022-05-10 14:20:53 [DEBUG] - 数据插入错误提示："duplicate key value violates unique constraint \"oauth_access_tokens_pkey\""
                    }
                    _ => {
                        log::error!("表(oauth_refresh_tokens)插入其它类型错误：{:#?}", error);
                    }
                }
                return None;
            }
        }
    }
}

pub fn get_refresh_token(refreshtoken: String) -> Option<OAuthRefreshTokens> {
    let query = oauth_refresh_tokens.find(refreshtoken);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!(
        "oauth_refresh_tokens查询（get_refresh_token）SQL：{:?}",
        sql
    );

    let connection = get_connection();
    let result = query.first::<OAuthRefreshTokens>(&connection);

    match result {
        Ok(data) => Some(data),
        Err(e) => {
            log::debug!("查无数据：{}", e);
            return None;
        }
    }
}

pub fn delete_refresh_token(refreshtoken: String) -> usize {
    let query = diesel::delete(oauth_refresh_tokens.find(refreshtoken));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("delete_refresh_token=>SQL：{:?}", sql);

    let connection = get_connection();
    let result = query.execute(&connection);

    log::debug!("删除返回{:#?}", result);
    // crate::common::type_v(&result); //变量值：Ok(1)  =>类型： &core::result::Result<usize, diesel::result::Error>

    result.unwrap_or(0)
}
