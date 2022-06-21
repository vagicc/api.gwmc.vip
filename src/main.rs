use crate::common::get_env;
use std::net::SocketAddr;

mod common;
mod db;
mod filters;
mod format_logger;
mod handlers;
mod models;
mod oauth;
mod routes;
mod schema;
mod template;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    //
    let log_level = crate::format_logger::get_log_level();

    println!("跟我买车-api日志记录级别：{}", log_level);

    //设置算定义日志记录格式  env_logger::init();  //RUST_LOG=debug cargo run
    // 自定义日志输出格式
    env_logger::Builder::new()
        .format(crate::format_logger::formatlog)
        .filter(None, log_level)
        .init();

    let routes = filters::all_routes();

    //取得https证书等
    let cert_path = get_env("cert_path");
    let key_path = get_env("key_path");
    let ip_addr = get_env("ip_address");
    let socket_addr: SocketAddr = ip_addr.as_str().parse().unwrap();

    log::info!("跟我买车-api: （{}）", get_env("BASE_URL"));
    log::info!("warp https 监听： {:?}", ip_addr);

    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(socket_addr)
        .await;
}
