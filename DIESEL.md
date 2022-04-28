数据库迁移使用说明

# diesel是Rust的ORM(对象关系映射器)和查询构建器
# diesel为PostgreSQL、Mysql及SQLite提供了开箱即用的支持

diesel数据库迁移使用说明
diesel是Rust的ORM(对象关系映射器)和查询构建器
diesel为PostgreSQL、Mysql及SQLite提供了开箱即用的支持
英文在线文档：https://lib.rs/crates/diesel_cli
diesel-cli命令行工具（创建、迁移）：

安装diesel-cli工具(postgres)：cargo install diesel_cli --no-default-features --features postgres
PostgreSQL错误解决的：sudo apt-get install libpq-dev

安装diesel-cli工具(mysql)：cargo install diesel_cli --no-default-features --features mysql
mysql错误解决：sudo apt-get install libmysqlclient-dev

在cargo项目根目录下添加.env文件,加下如下条进行连接配置：
postgres数据库：
DATABASE_URL=postgres://postgres:llxxs@127.0.0.1:5432/linksnap
mysql数据库：
DATABASE_URL=mysql://[user[:password]@]host/database_name

在Cargo.toml中添加依赖项：
diesel = { version="1.4.6",features=["extras","postgres","r2d2"] }
dotenv = "0.15.0"

运行"diesel setup"命令生成"migrations"目录与"diesel.toml"文件并且会创建数据库：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel setup
Creating migrations directory at: /luck/Language/Rust/warp-wiki/migrations
Creating database: warpwiki
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$

创建admins表迁移，运行创建表迁移命令（diesel migration generate 表名）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration generate admins
Creating migrations/2021-05-13-071702_admins/up.sql
Creating migrations/2021-05-13-071702_admins/down.sql
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ 
命令运行后会生成两个空的迁移文件up.sql和down.sql,
迁移文件只是普通的SQL,接着在up.sql上面添加CREATE TABLE,同时在down.sql添加相应的DROP TABLE

执行表迁移命令（diesel migration run）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration run
Running migration 2021-05-13-071702_admins
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$
命令执行完后，会在数据库中生成表，同时在项目中生成src/schema.rs文件。


迁移时执行：diesel setup

同时会在src/schema.rs中添加相应的(table!宏)表结构.


运行时出错：error while loading shared libraries: libmariadb.so.3: cannot open shared object file: No such file or directory
解决： sudo ln -s /usr/local/mysql/lib/libmariadb.so.3 /usr/lib/libmariadb.so.3

-------------------------“重做”应用的迁移--------
应用迁移：diesel migration run  
恢复迁移：diesel migration revert
“重做”应用的迁移：
          diesel migration revert
          diesel migration run
重做（等同于上面两条）：diesel migration redo
上面命令，只能运行、还原或重做一次迁移
重做所有的迁移：diesel database reset 
diesel database reset 这条命令执行后会删除数据库，然后按照迁移文件创建数据库和表等。
-----------------------------------------------

不会使用时，可以查看diesel源码里的示例：
查询条件看：src/expression_methods/global_expression_methods.rs
事务查看：
    src/connection/mod.rs  事务写在一个函数的示例
    src/doctest_setup.rs
    src/connection/transaction_manager.rs  事务
    src/pg/transaction.rs   pg数据库特有的事务