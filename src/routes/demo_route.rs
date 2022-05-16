use crate::handlers::demo_handler;
use warp::Filter;

/*
返回本页所有路由
1、POST /url 创建
2、DELETE /url/xxx 删除
3、PUT /url/xxx 更新
4、GET /url/xxx 查看
*/
pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let demo_all = list()
        .or(get_demo())
        .or(add())
        .or(do_add())
        .or(delete())
        .or(update())
        .or(test_token())
        .or(demo_login())
        .or(refresh_token());
    demo_all
}

/* 取得demo表数据列表 */
pub fn list() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let list = warp::get()
        .and(warp::path("demo"))
        .and(warp::path::end())
        .and_then(demo_handler::list);
    list
}

/* 查 */
pub fn get_demo() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("demo"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(demo_handler::get_demo)
}

pub fn add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add = warp::get()
        .and(warp::path!("demo" / "add"))
        .and(warp::path::end())
        .and_then(demo_handler::add_form);
    add
}

/* 增 */
pub fn do_add() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let do_add_yl = warp::post()
        .and(warp::path!("demo" / "add"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(demo_handler::do_add);
    let do_add = warp::post()
        .and(warp::path("demo"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(demo_handler::do_add);
    do_add.or(do_add_yl)
}

/* 删 */
pub fn delete() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let delete = warp::delete()
        .and(warp::path("demo"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(demo_handler::delete_demo);
    delete
}

/* 改 */
pub fn update() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let update = warp::put()
        .and(warp::path("demo"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(demo_handler::update_demo);
    update
}

pub fn test_token() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let test = warp::get()
        .and(warp::path!("demo" / "check_token"))
        .and(crate::oauth::auth())
        .and(warp::path::end())
        .and_then(demo_handler::test_token);
    test
}

// 测试登录产生token
pub fn demo_login() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("demo" / "login"))
        .and(warp::path::end())
        .and(warp::body::form())
        .and_then(demo_handler::demo_login)
}

pub fn refresh_token() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let refresh = warp::post()
        .and(warp::path!("demo" / "refresh_token"))
        .and(warp::path::end())
        .and(warp::header::value("Authorization"))
        .and(warp::body::form())
        .and_then(demo_handler::refresh_token);
     
    refresh
}
