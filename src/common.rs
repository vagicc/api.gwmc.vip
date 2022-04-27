use dotenv::dotenv;
use std::env;

/// 取得.env文件key里的值
pub fn get_env(key: &str) -> String {
    dotenv().ok();
    let msg = ".env文件必须配置的环境变量：".to_string() + key;
    let value = env::var(key).expect(&msg);
    value
}
