use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{pg::PgConnection, Connection};
// use diesel::{mysql::MysqlConnection, Connection};

use crate::common::get_env;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;
// pub type DBConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

/* 
2022-06-21 09:21:49 [ERROR] - connection to server at "127.0.0.1", port 5432 failed: Connection refused
	Is the server running on that host and accepting TCP/IP connections?

thread 'tokio-runtime-worker' panicked at '数据库连接出错: Error(Some("connection to server at \"127.0.0.1\", port 5432 failed: Connection refused\n\tIs the server running on that host and accepting TCP/IP connections?\n"))', src/db.rs:13:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/
// 所有的数据库操作都应使用些处获取连接
pub fn get_connection() -> DBConnection {
    DB_CONN_POOL.get().expect("数据库连接出错")
}

lazy_static::lazy_static! {
    pub static ref DB_CONN_POOL:DBPool=establish_connection();
}

/* 使用diesel中R2D2连接池 */
pub fn establish_connection() -> DBPool {
    let database_url = get_env("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("diesel::r2d2::Pool连接Postgres数据库出错");
    pool
}

/* 这个使用diesel中R2D2连接池:旧====暂先不删除，这里设置了最大连接数 */
pub fn _establish_connection() -> DBConnection {
    let database_url = get_env("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // let pool = Pool::new(manager).expect("diesel::r2d2::Pool连接Postgres数据库出错");
    // let conn = pool.get().expect("数据库连接出错");

    let pool = Pool::builder()
        .max_size(158)
        .test_on_check_out(true)
        .build(manager)
        .expect("diesel::r2d2::Pool连接Postgres数据库出错");

    //这个是否要先
    let conn = pool.get().expect("数据库连接出错");
    conn
}

pub fn _pg_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = get_env("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("无法创建Postgres连接池");
    pool
}

pub fn _my_pg_connection() -> PgConnection {
    let database_url = get_env("DATABASE_URL");
    let conn = PgConnection::establish(&database_url).expect("kdka");
    conn
}
