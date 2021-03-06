use crate::common::response_json;
use crate::models::demo_model;
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
