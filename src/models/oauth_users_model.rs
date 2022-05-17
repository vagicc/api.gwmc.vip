use crate::db::get_connection;
use crate::schema::oauth_users;
use crate::schema::oauth_users::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/* 表查询插入结构体合二为一(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Insertable, Queryable, Deserialize, Serialize)]
#[table_name = "oauth_users"]
pub struct OAuthUsers {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub salt: Option<String>,
    pub scope: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
}
impl OAuthUsers {
    pub fn insert(&mut self) -> Option<OAuthUsers> {
        /* 处理expires有效时间 */
        if self.create_time.is_none() {
            let now_date_time = crate::common::now_naive_date_time();
            self.create_time = Some(now_date_time);
        }

        let connection = get_connection();
        let insert_data = diesel::insert_into(oauth_users)
            .values(self.clone())
            .get_result::<OAuthUsers>(&connection); //这里还要处理，不能中断程序

        match insert_data {
            Ok(data) => {
                log::debug!("表(oauth_users)成功插入数据：{:?}", data);
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
                        log::debug!("表(oauth_users)插入错误：{:#?}", error_kind); //2022-05-10 14:20:53 [DEBUG] - 数据插入错误：UniqueViolation
                        log::debug!("表(oauth_users)插入错误提示：{:#?}", message);
                    }
                    _ => {
                        log::error!("表(oauth_users)插入其它类型错误：{:#?}", database_error);
                    }
                }
                return None;
            }
        }
    }
}

pub fn get_oauth_user(user_name: String)->Option<OAuthUsers> {
    let query = oauth_users.filter(username.eq(user_name));

    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_oauth_user查询SQL：{:?}", sql);

    let connection = get_connection();
    let result = query.first::<OAuthUsers>(&connection);

    match result {
        Ok(users) => return Some(users),
        Err(err) => {
            log::debug!("oauth_users查无数据：{:?}", err);
            return None;
        }
    }
}

/* 加盐 */
pub fn get_new_salt() -> String {
    let new_salt = crate::common::random_key(10);
    new_salt
}

/* 密码加密算法 */
pub fn encryption(passwd: &String, new_salt: &String) -> String {
    let new_passwd = format!("{}luck{}", passwd, new_salt).to_owned();
    let sha1_passwd = get_sha1(&new_passwd);
    sha1_passwd
}

pub fn get_sha1(passwd: &str) -> String {
    let sha1_string = sha1::Sha1::from(passwd).digest().to_string();
    println!("Sha1加密后：{}", sha1_string);
    println!("Sha1加密后长度：{}", sha1_string.len());
    sha1_string
}
