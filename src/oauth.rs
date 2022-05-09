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
pub fn check_oauth_client(client_id: String, client_secret: String, grant_type: String) -> bool {
    return true;
}

fn get_oauth_clients(client_id: String) {}

//  刷新流程 查找oauth_refresh_tokens后校验时间，并删除当前条。    写入oauth_access_tokens oauth_refresh_tokens
pub fn refresh(refresh_token: String) -> Option<Token> {
    if !check_refresh_token(refresh_token.clone()) {
        log::error!("refresh_token无效");
        return None;
    }

    let k = delete_refresh_token(refresh_token);
    let (token, refresh_token) = new_token_and_refresh_token();

    return Some(Token {
        access_token: token,
        expires_in: 3600,
        token_type: "Bearer".to_string(),
        scope: "app".to_string(),
        refresh_token: refresh_token,
    });
}

fn delete_refresh_token(refresh_token: String) -> bool {
    return true; //删除成功
}

// 查找oauth_refresh_tokens后校验时间
fn check_refresh_token(refresh_token: String) -> bool {
    return true;
}

// 登录流程 校验 oauth_clients 再校验 oauth_users             写入oauth_access_tokens oauth_refresh_tokens
pub fn new_token(userid: i32) -> Token {
    let (token, refresh_token) = new_token_and_refresh_token();
    return Token {
        access_token: token,
        expires_in: 3600,
        token_type: "Bearer".to_string(),
        scope: "app".to_string(),
        refresh_token: refresh_token,
    };
}

/* 写入oauth_access_tokens oauth_refresh_tokens */
fn new_token_and_refresh_token() -> (String, String) {
    let token = create_random_token();
    let refresh_token = create_random_token();

    (token, refresh_token)
}

// 查表oauth_access_tokens，取得用户ID
fn get_user_id(token: &str) -> i32 {
    let user_id: i32 = 89;
    user_id
}

//  创建字符串token
pub fn create_random_token() -> String {
    // let token = String::new();
    let token = crate::common::random_key(33); //这里设置产生的token长度
    println!("随机：{}", token);  //KNT14Wi9XzObHKjo7fNCFiPl104bQQk56
    println!("长度：{}", token.len());

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
    let tep = match token {
        Some(token) => {
            log::debug!("头带有token");
            /* 通过头token去查表里的token,判断是否在有效期内 */
            let k: i32 = get_user_id(token); //返回用户ID
            Ok(k)
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
    };
    tep
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
