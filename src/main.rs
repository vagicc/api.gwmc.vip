use crate::common::get_env;
use std::io::Write;
use std::net::SocketAddr;

mod common;
mod filters;
mod handlers;
mod routes;
mod db;
mod schema;
mod models;
mod template;
mod oauth;

#[macro_use]
extern crate diesel;

#[tokio::main]
async fn main() {
    //
    let log_level = get_log_level();

    println!("跟我买车-api日志记录级别：{}", log_level);

    //设置算定义日志记录格式  env_logger::init();  //RUST_LOG=debug cargo run
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
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

fn get_log_level() -> log::LevelFilter {
    let level = get_env("RUST_LOG");

    let mut log_level = log::LevelFilter::Debug;
    if level.eq("Debug") {
        log_level = log::LevelFilter::Debug;
    } else if level.eq("Info") {
        log_level = log::LevelFilter::Info;
    } else if level.eq("Warn") {
        log_level = log::LevelFilter::Warn;
    } else if level.eq("Error") {
        log_level = log::LevelFilter::Error;
    } else if level.eq("Off") {
        log_level = log::LevelFilter::Off;
    } else if level.eq("Trace") {
        log_level = log::LevelFilter::Trace;
    }
    log_level
}
