use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

/* 响应/请求的返回 */
pub async fn index() -> ResultWarp<impl Reply> {
    let html = "欢迎访问首页(Hi Luck)";
    Ok(warp::reply::html(html))    //直接返回html
    // Err(warp::reject::not_found())   //错误的返回
}
