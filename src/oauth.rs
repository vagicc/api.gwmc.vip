use crate::models::oauth_access_tokens_model;
use crate::models::oauth_clients_model;
use crate::models::oauth_clients_model::OAuthClients;
use crate::models::oauth_refresh_tokens_model;
use serde::{Deserialize, Serialize};
use warp::http::header::HeaderMap;
use warp::http::header::HeaderValue;
use warp::http::header::AUTHORIZATION;
use warp::http::Response;
use warp::Filter;
/*
OAuth2.0
--  登录流程 校验 oauth_clients 再校验 oauth_users             写入oauth_access_tokens oauth_refresh_tokens
--  刷新流程 查找oauth_refresh_tokens后校验时间，并删除当前条。    写入oauth_access_tokens oauth_refresh_tokens
登录：创建token
接口：验证token
刷新：token

 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i32,    //有效时长： 3600
    pub token_type: String, //固定为：Bearer
    pub scope: String,      //暂为app
    pub refresh_token: String,
}

// 校验 oauth_clients的客户端密钥与grant_type是否有此类型
pub fn check_oauth_client(
    client_id: String,
    _client_secret: String,
    _grant_type: String,
) -> Option<OAuthClients> {
    if let Some(client) = oauth_clients_model::get_client(client_id) {
        if let Some(_secret) = &client.client_secret {
            log::debug!("开始检验client secret,这里还没做检验………………");
        }

        return Some(client.clone());
    } else {
        return None;
    }
}

fn _get_oauth_clients(_client_id: String) {}

//刷新流程 查找oauth_refresh_tokens后校验时间，并删除当前条。    写入oauth_access_tokens oauth_refresh_tokens
pub fn refresh(
    refresh_token: String,
    old_token: &str,
    client: &OAuthClients,
) -> Result<Token, &'static str> {
    let option_user_id = check_refresh_token(refresh_token.clone());
    if option_user_id.is_none() {
        return Err("refresh_token无效");
    }

    let userid = option_user_id.unwrap();

    if let Some((user_id, _)) = get_user_and_expires(old_token) {
        if userid != user_id {
            log::debug!("token与refresh_token非同一用户");
            return Err("token与refresh_token非同一用户");
        }
    } else {
        return Err("旧token无效（header）");
    }

    let (token, new_refresh_token) = new_token_and_refresh_token(userid, client);
    let _ = delete_refresh_token(refresh_token); //删除原来的refresh_token
    //  是否要删除原来的 old_token

    return Ok(Token {
        access_token: token,
        expires_in: 3600,
        token_type: "Bearer".to_string(),
        scope: client.scope.clone().unwrap_or("app".to_string()),
        refresh_token: new_refresh_token,
    });
}

fn delete_refresh_token(refresh_token: String) -> bool {
    let delete_count = oauth_refresh_tokens_model::delete_refresh_token(refresh_token);

    if delete_count < 1 {
        return false;
    }

    return true; //删除成功
}

// 查找oauth_refresh_tokens后校验时间
fn check_refresh_token(refresh_token: String) -> Option<i32> {
    let refresh = oauth_refresh_tokens_model::get_refresh_token(refresh_token);

    if refresh.is_none() {
        return None;
    }
    let refresh = refresh.unwrap();
    let now_date_time = crate::common::now_naive_date_time(); //当前时间
    if refresh.expires.unwrap() < now_date_time {
        log::debug!("refresh_token已失效!!");
        return None;
    }

    Some(refresh.user_id)
}

// 登录流程 校验 oauth_clients 再校验 oauth_users             写入oauth_access_tokens oauth_refresh_tokens
pub fn new_token(userid: i32, client: &OAuthClients) -> Token {
    // 如果要做“单点登录”，在此做……
    let (token, refresh_token) = new_token_and_refresh_token(userid, client);

    return Token {
        access_token: token,
        expires_in: 3600,
        token_type: "Bearer".to_string(),
        scope: client.scope.clone().unwrap_or("app".to_string()),
        refresh_token: refresh_token,
    };
}

/* 写入oauth_access_tokens 与 oauth_refresh_tokens */
fn new_token_and_refresh_token(userid: i32, client: &OAuthClients) -> (String, String) {
    let expires = get_expires(false); //密钥有效时间
    let refresh_expires = get_expires(true); //刷新密钥有效时间

    // let mut token = "OxwAmknIVkyeZ9WeGLRjpCSCegdlk5CcgtRD7q".to_string();
    // let mut refresh_token = "CPgWuTLeuXMaMpxfOMMNqikGATSGf0bYhryzwQ".to_string();
    let mut token = create_random_token();
    let mut refresh_token = create_random_token();

    let mut access_token = oauth_access_tokens_model::OAuthAccessTokens {
        access_token: token.clone(),
        client_id: client.client_id.clone(),
        user_id: userid,
        expires: Some(expires),
        scope: client.scope.clone(),
    };

    log::debug!("插入oauth_access_tokens表 token有效期为一个小时");
    let mut oauth_access_token = access_token.insert(); //插入表
    if oauth_access_token.is_none() {
        log::debug!("第一次插入oauth_access_tokens：{:#?}", access_token);
        log::debug!("插入oauth_access_tokens出意外,换个token再来一次");
        token = refresh_token.clone();
        refresh_token = access_token.access_token.clone();
        access_token.access_token = token.clone();
        oauth_access_token = access_token.insert(); //插入表
        log::debug!("第二次插入oauth_access_tokens：{:#?}", access_token);
        if oauth_access_token.is_none() {
            log::error!("两次插入oauth_access_tokens都失败了");
        }
    }

    log::debug!("插入oauth_refresh_tokens表 8天内可刷新");
    let mut refresh_insert_data = oauth_refresh_tokens_model::OAuthRefreshTokens {
        refresh_token: refresh_token.clone(),
        client_id: client.client_id.clone(),
        user_id: userid,
        expires: Some(refresh_expires),
        scope: client.scope.clone(),
    };
    let mut oauth_refresh_tokens = refresh_insert_data.insert();
    if oauth_refresh_tokens.is_none() {
        log::debug!("插入oauth_refresh_tokens出意外,换个refresh_token再来一次");
        refresh_token = create_random_token();
        refresh_insert_data.refresh_token = refresh_token.clone();
        oauth_refresh_tokens = refresh_insert_data.insert();
        if oauth_refresh_tokens.is_none() {
            log::error!("两次插入oauth_refresh_tokens都失败了");
        }
    }

    (token, refresh_token)
}

// 取得有效时间 ,这里可整改一次返回token有效时间和可刷新有效时间
fn get_expires(is_refresh: bool) -> chrono::NaiveDateTime {
    use chrono::TimeZone;
    // use chrono::prelude::{Local, NaiveDate, NaiveDateTime};
    let exp = if is_refresh { 3600 * 24 * 8 } else { 3600 };
    // let exp = 3600; //有效3600秒,也就是一个钟

    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = chrono::prelude::Local::now();

    let mills = now.timestamp_millis(); //毫秒时间戳
    let mills_exp = mills + exp * 1000; //有效时间(秒×1000=毫秒)
    let expires = chrono::prelude::Local.timestamp_millis(mills_exp);
    let expires_dft = expires.format(fmt);
    let expires_str_date = expires_dft.to_string();
    // println!("有效时间：{}", expires_str_date);
    let expires_date_time =
        chrono::prelude::NaiveDateTime::parse_from_str(expires_str_date.as_str(), fmt).unwrap();
    // println!("NaiveDateTime:{:#?}", expires_date_time);
    return expires_date_time;

    // let dft = now.format(fmt);
    // let str_date = dft.to_string();
    // println!("当前时间：{}", str_date);
    // let now_date_time =
    //     chrono::prelude::NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();
    // // let now_date = chrono::prelude::NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d").unwrap();

    // println!("判断时间大小");
    // if expires_date_time > now_date_time {
    //     println!("还在有效期");
    // }
}

// 查表oauth_access_tokens，取得用户ID
fn get_user_and_expires(token: &str) -> Option<(i32, chrono::NaiveDateTime)> {
    match oauth_access_tokens_model::get_user_id(token) {
        Some(oauth_access_token) => {
            //放在这里校验token是否过了有效时间？？,放在调用上做判断
            let user_and_expires = (
                oauth_access_token.user_id,
                oauth_access_token.expires.unwrap(),
            );

            return Some(user_and_expires);
        }
        None => {
            return None;
        }
    }

    // let user_id: i32 = 89;
    // user_id
}

//  创建字符串token
pub fn create_random_token() -> String {
    // let token = String::new();
    let token = crate::common::random_key(38); //这里设置产生的token长度
                                               // println!("随机：{}", token);  //KNT14Wi9XzObHKjo7fNCFiPl104bQQk56
                                               // println!("长度：{}", token.len());

    token
}

/* 头部授权 */
pub fn auth() -> impl Filter<Extract = (i32,), Error = warp::Rejection> + Clone {
    warp::header::headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and_then(authorize)
}

/* 头部验证授权：验证token,成功返回用户ID */
async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<i32, warp::Rejection> {
    let token = from_header(&headers);
    match token {
        Some(token) => {
            log::debug!("头带有token:{}", token);
            /* 通过头token去查表里的token,判断是否在有效期内 */
            if let Some((user_id, expires)) = get_user_and_expires(token) {
                //放在这里校验token是否过了有效时间？？   -----------------=================这里还没有做=========----
                let now_date_time = crate::common::now_naive_date_time(); //当前时间
                if expires < now_date_time {
                    log::debug!(
                        "token过了有效期:expires({}) < now({}) ",
                        expires,
                        now_date_time
                    );
                    let my = ResponseError {
                        error: "token失效期".to_string(),
                        error_description: "==token过了有效期==ddd".to_string(),
                    };

                    return Err(warp::reject::custom(my));
                }
                return Ok(user_id);
            }

            log::debug!("token无对应用户，无效token");
            let my = ResponseError {
                error: "无效token".to_string(),
                error_description: "==无用户对应token".to_string(),
            };

            return Err(warp::reject::custom(my));
        }
        None => {
            log::debug!("头无token，无权限");
            //(511, NETWORK_AUTHENTICATION_REQUIRED, "Network Authentication Required");
            let my = ResponseError {
                error: "头".to_string(),
                error_description: "==头无token==ddd".to_string(),
            };
            let emy = Err(warp::reject::custom(my));

            return emy;

            // let r = Err(warp::reject::custom(MethodError));
            // return r;
        }
    }
}

/* 取得头部传过来的token */
fn from_header(headers: &HeaderMap<HeaderValue>) -> Option<&str> {
    let header = match headers.get(AUTHORIZATION) {
        Some(h) => h,
        None => return None,
    };

    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(token) => token,
        Err(_) => return None,
    };

    if auth_header.is_empty() || !auth_header.starts_with("Bearer ") {
        return None;
    }

    let token = auth_header.trim_start_matches("Bearer ");
    Some(token)
}

#[derive(Debug)]
struct MethodError;
impl warp::reject::Reject for MethodError {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseError {
    pub error: String,
    pub error_description: String,
}
impl warp::reject::Reject for ResponseError {
    // into_response  //warp:src/reject.rs   448行
}

impl warp::Reply for ResponseError {
    fn into_response(self) -> warp::reply::Response {
        Response::new(
            format!(
                "出错提示message: {} =>description:{}",
                self.error, self.error_description
            )
            .into(),
        )
    }
}

// pub fn response_error(error: String, description: String) -> Result<(), ResponseError> {
//     let response = ResponseError {
//         error: error,
//         error_description: description,
//     };
//     let kd=Err(response);

//     Err(response)
// }
