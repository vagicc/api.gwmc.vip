use crate::handlers::demo_handler;
use warp::{filters::BoxedFilter, Filter};

/* 返回本页所有路由 */
pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let demo_all = add().or(do_add());
    demo_all
}

/* 取得demo表数据列表 */
pub fn list() {
    let list = warp::get().and(warp::path("demo")).and(warp::path::end());
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add = warp::get()
        .and(warp::path!("demo" / "add"))
        .and(warp::path::end())
        .and_then(demo_handler::add_form);
    add
}

pub fn do_add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let do_add = warp::post()
        .and(warp::path!("demo" / "add"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(demo_handler::do_add);
    do_add
}
