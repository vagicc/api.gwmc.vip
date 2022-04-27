use crate::handlers::home_handler;
use warp::Filter;

/* 访问站点 / 时的路由 */
pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let home = warp::get()
        .and(warp::path::end())
        .and_then(home_handler::index);
    home
}
