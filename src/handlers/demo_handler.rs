use crate::common::response_json;
use crate::models::demo_model;
use crate::models::oauth_users_model;
use crate::template::to_html_single;
use handlebars::to_json;
use serde::{Deserialize, Serialize};
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

/*  */
pub async fn list() -> ResultWarp<impl Reply> {
    log::debug!("取得demo表数据");
    let list = demo_model::get_list();
    response_json(warp::http::StatusCode::OK, Some(&list), None)
}

pub async fn get_demo(id: i32) -> ResultWarp<impl Reply> {
    log::debug!("取得demo单条数据");
    let demo = demo_model::get_demo(id);

    response_json(warp::http::StatusCode::OK, Some(&demo), None)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateForm {
    pub name: String,
}
pub async fn update_demo(id: i32, form: UpdateForm) -> ResultWarp<impl Reply> {
    log::debug!("修改demo单条数据");
    let update = demo_model::update(id, form);
    response_json(warp::http::StatusCode::OK, Some(&update), None)
}

pub async fn delete_demo(id: i32) -> ResultWarp<impl Reply> {
    log::debug!("删除demo单条数据");
    let delete_count = demo_model::delete(id);

    let mut message = String::from("删除失败");
    if delete_count > 0 {
        message = String::from("删除成功");
    }

    response_json(
        warp::http::StatusCode::OK,
        Some(&delete_count),
        Some(message),
    )
}

/* 输出html表单 */
pub async fn add_form() -> ResultWarp<impl Reply> {
    log::debug!("[调试信息]响应GET请求（/demo/add），输出html");
    // log::warn!("[警告信息] warn");
    // log::info!("[提示信息] info");
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("增加数据到demo表"));
    let html = to_html_single("newdemo.html", data);

    // Err(warp::reject::not_found())
    Ok(warp::reply::html(html))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddDemoForm {
    id: i32,
    name: String,
}

impl AddDemoForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.id <= 0 {
            return Err("ID不能小于");
        }
        if self.name.is_empty() {
            return Err("name不能为空");
        }
        if self.name.len() < 2 {
            return Err("name长度应大于2");
        }
        Ok(self.clone())
    }
}

pub async fn do_add(form: AddDemoForm) -> ResultWarp<impl Reply> {
    log::debug!("[调试信息]响应POST请求（/demo/add），处理表单数据，并添加到demo表");
    // let result = form.validate();
    let result = form.validate().map_err(|op| op.to_string());

    // let mut status_code = warp::http::StatusCode::OK;

    match result {
        Ok(form) => {
            log::debug!("[调试信息] 新增数据{:?}", form);
            let mut insert_data = demo_model::NewDemo {
                name: form.name,
                create_time: None,
            };
            let data = insert_data.insert();

            let status_code = warp::http::StatusCode::CREATED;

            let response = response_json(status_code, Some(&data), None);

            // let te = serde_json::to_string(&data).unwrap();
            // let response = response_json_old(status_code, te);

            return response;
        }
        Err(e) => {
            log::error!("[错误信息] {}", e);
            let status_code = warp::http::StatusCode::ACCEPTED;

            let response = response_json(status_code, None, Some(e));
            // let response = response_json_old(status_code, e);
            return response;
        }
    }

    /* 返回JSON，并且设置了状态码与头 */
    // Ok(warp::http::Response::builder()
    //     .status(warp::http::StatusCode::CREATED)
    //     .header("Content-type", "application/json")
    //     .body(serde_json::to_string(&form).unwrap()))

    // Ok(warp::reply::html("新增成功"))  //返回html
}

pub async fn test_token(uid: i32) -> ResultWarp<impl Reply> {
    log::debug!("来到了私有页面了!!user_id:{}", uid);

    Ok(warp::reply::html("token验证通过，有权限访问")) //返回html
}

// {
//     "error": "invalid_grant",
//     "error_description": "Invalid username and password combination"
// }
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorMessage {
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DemoLogin {
    pub username: String,      //登录用户名
    pub password: String,      //登录密码
    pub grant_type: String, //说明：grant_type有password、client_credentials、refresh_token、authorization_code
    pub client_id: String,  //登录客户端
    pub client_secret: String, //客户端密钥
}

impl DemoLogin {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            return Err("username不能为空");
        }
        if self.password.len() < 2 {
            return Err("password长度应大于2");
        }
        if self.grant_type.is_empty() {
            return Err("grant_type不能为空");
        }
        if self.client_id.is_empty() {
            return Err("client_id不能为空");
        }
        Ok(self.clone())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SignupForm {
    pub username: String,
    pub password: String,
    pub passwd: String,
}
impl SignupForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            Err("手机号不能为空")
        } else if self.password.is_empty() {
            Err("密码不能为空")
        } else if self.password.len() < 5 || self.passwd.len() < 5 {
            Err("密码与确认密码的长度不能小于5位")
        } else if self.password != self.passwd {
            Err("密码与确认密码必须一致")
        } else {
            Ok(self.clone())
        }
    }
}
// 处理注册
pub async fn demo_do_signup(form: SignupForm) -> ResultWarp<impl Reply> {
    log::debug!("开始处理注册");
    match form.validate() {
        Ok(signup) => {
            let salt = oauth_users_model::get_new_salt();
            let pwd = oauth_users_model::encryption(&signup.passwd, &salt);

            let mut new_users = oauth_users_model::OAuthUsers {
                user_id: 8,
                username: signup.username,
                password: pwd,
                salt: Some(salt),
                scope: None,
                create_time: None,
                last_login: None,
            };
            let _tem = new_users.insert();
        }
        Err(message) => {
            println!("{:?}", message);
        }
    }

    Ok(warp::reply::html("注册成功")) //返回html
}

pub async fn demo_login(login: DemoLogin) -> ResultWarp<impl Reply> {
    let validate = login.validate();
    match validate {
        Ok(form) => {
            // 校验密码 ，得到用户ID
            let users = oauth_users_model::get_oauth_user(form.username);
            if users.is_none() {
                let response = ErrorMessage {
                    error: "无此用户".to_string(),
                    error_description: "查无此用户,登录失败".to_string(),
                };
                return Ok(warp::http::Response::builder()
                    .status(warp::http::StatusCode::MOVED_PERMANENTLY)
                    .header("Content-type", "application/json")
                    .body(serde_json::to_string(&response).unwrap()));
            }

            let users = users.unwrap();

            /* 判断用户密码 */
            let pwd = oauth_users_model::encryption(
                &form.password,
                &users.salt.unwrap_or("".to_string()),
            );
            if !pwd.eq(&users.password) {
                let response = ErrorMessage {
                    error: "用户密码不正确".to_string(),
                    error_description: "用户错误，请重新输入用户与密码".to_string(),
                };
                return Ok(warp::http::Response::builder()
                    .status(warp::http::StatusCode::MOVED_PERMANENTLY)
                    .header("Content-type", "application/json")
                    .body(serde_json::to_string(&response).unwrap()));
            }

            let user_id: i32 = users.user_id;
            // 通过用户ID，生成已登录token
            use crate::oauth;
            //先校验客户端
            let client =
                oauth::check_oauth_client(form.client_id, form.client_secret, form.grant_type);
            if client.is_none() {
                log::debug!("客户端验证不通过,登录失败");
                let response = ErrorMessage {
                    error: "客户端验证".to_string(),
                    error_description: "客户端验证不通过,登录失败".to_string(),
                };
                return Ok(warp::http::Response::builder()
                    .status(warp::http::StatusCode::MOVED_PERMANENTLY)
                    .header("Content-type", "application/json")
                    .body(serde_json::to_string(&response).unwrap()));
            }

            let client = client.unwrap();

            let token = oauth::new_token(user_id, &client);

            /* 返回JSON，并且设置了状态码与头 */
            return Ok(warp::http::Response::builder()
                .status(warp::http::StatusCode::OK)
                .header("Content-type", "application/json")
                .body(serde_json::to_string(&token).unwrap()));
        }
        Err(e) => {
            // return Ok(warp::http::Response::builder()
            //     .status(warp::http::StatusCode::MOVED_PERMANENTLY)
            //     .header("Content-type", "application/json")
            //     .body(e.to_string()));

            let response = ErrorMessage {
                error: e.to_string(),
                error_description: e.to_string(),
            };
            return Ok(warp::http::Response::builder()
                .status(warp::http::StatusCode::MOVED_PERMANENTLY)
                .header("Content-type", "application/json")
                .body(serde_json::to_string(&response).unwrap()));
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RefreshForm {
    pub grant_type: String, //说明：grant_type有password、client_credentials、refresh_token、authorization_code
    pub refresh_token: String,
    pub client_id: String,     //登录客户端
    pub client_secret: String, //客户端密钥
}
impl RefreshForm {
    pub fn validate(&self) -> Result<Self, &'static str> {
        if self.grant_type.is_empty() {
            return Err("grant_type不能为空");
        }
        if !self.grant_type.eq("refresh_token") {
            return Err("grant_type类型错误");
        }
        if self.refresh_token.is_empty() {
            return Err("refresh_token不能为空");
        }
        if self.refresh_token.len() != 38 {
            return Err("refresh_token长度不符合要求");
        }
        if self.client_id.is_empty() {
            return Err("client_id不能为空");
        }
        Ok(self.clone())
    }
}

pub async fn refresh_token(
    token: warp::http::header::HeaderValue, //头部带的token
    refresh_form: RefreshForm,
) -> ResultWarp<impl Reply> {
    log::debug!(
        "用户请求刷新token！token:{:?},form:{:?}",
        token,
        refresh_form
    );

    match refresh_form.validate() {
        Ok(form) => {
            use crate::oauth;
            //先校验客户端
            let client =
                oauth::check_oauth_client(form.client_id, form.client_secret, form.grant_type);
            if client.is_none() {
                log::debug!("客户端验证不通过,刷新token失败");
            }

            let client = client.unwrap();

            let old_token = match std::str::from_utf8(token.as_bytes()) {
                Ok(token) => token.trim_start_matches("Bearer "),
                Err(_) => "",
            };

            match oauth::refresh(form.refresh_token, old_token, &client) {
                Ok(new_token) => {
                    return Ok(warp::http::Response::builder()
                        .status(warp::http::StatusCode::OK)
                        .header("Content-type", "application/json")
                        .body(serde_json::to_string(&new_token).unwrap()))
                }
                Err(message) => {
                    let response = ErrorMessage {
                        error: message.to_string(),
                        error_description: message.to_string(),
                    };
                    return Ok(warp::http::Response::builder()
                        .status(warp::http::StatusCode::MOVED_PERMANENTLY)
                        .header("Content-type", "application/json")
                        .body(serde_json::to_string(&response).unwrap()));
                    // return Ok(warp::http::Response::builder()
                    //     .status(warp::http::StatusCode::MOVED_PERMANENTLY)
                    //     .header("Content-type", "application/json")
                    //     .body(message.to_string()))
                }
            }
        }
        Err(e) => {
            let response = ErrorMessage {
                error: e.to_string(),
                error_description: e.to_string(),
            };
            return Ok(warp::http::Response::builder()
                .status(warp::http::StatusCode::MOVED_PERMANENTLY)
                .header("Content-type", "application/json")
                .body(serde_json::to_string(&response).unwrap()));
        }
    }

    // Ok(warp::reply::html("新增成功")) //返回html
}
