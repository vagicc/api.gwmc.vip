use crate::db::get_connection;
use crate::schema::demo;
use crate::schema::demo::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, PartialEq, Eq, Deserialize, Serialize)]
pub struct Demo {
    pub id: i32,
    pub name: String,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[table_name = "demo"]
pub struct NewDemo {
    pub name: String,
    pub create_time: Option<NaiveDateTime>,
}

impl NewDemo {
    pub fn insert(&mut self) -> Demo {
        /* 处理创建时间 */
        if self.create_time.is_none() {
            // use chrono::prelude::{Local, NaiveDate, NaiveDateTime};
            let fmt = "%Y-%m-%d %H:%M:%S";
            let now = chrono::prelude::Local::now();
            let dft = now.format(fmt);
            let str_date = dft.to_string();
            // println!("当前时间：{}", str_date);
            let now_date_time =
                chrono::prelude::NaiveDateTime::parse_from_str(str_date.as_str(), fmt).unwrap();
            // let now_date = chrono::prelude::NaiveDate::parse_from_str(str_date.as_str(), "%Y-%m-%d").unwrap();

            self.create_time = Some(now_date_time);
        }

        let connection = get_connection();
        let result = diesel::insert_into(demo)
            .values(self.clone())
            .get_result::<Demo>(&connection)
            .unwrap();
        println!("插入数据返回：{:#?}", result);
        result
    }
}

/* 返回所有数据 */
pub fn get_list() -> Vec<Demo> {
    let connection = get_connection();
    let query = demo.get_results::<Demo>(&connection).unwrap_or_else(|_op| {
        let temp: Vec<Demo> = Vec::new();
        return temp;
    });
    query
}

pub fn get_demo(demo_id: i32) -> Option<Demo> {
    let query = demo.find(demo_id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    println!("查询SQL：{:?}", sql);

    let connection = get_connection();
    let result = query.first::<Demo>(&connection);

    match result {
        Ok(data) => Some(data),
        Err(error) => {
            log::debug!("{}", error);
            return None;
        }
    }
}

pub fn delete(demo_id: i32)->usize {
    let query = diesel::delete(demo.find(demo_id));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("查询SQL：{:?}", sql);

    let connection = get_connection();
    let result = query.execute(&connection);

    log::debug!("删除返回{:#?}", result);
    crate::common::type_v(&result);  //变量值：Ok(1)  =>类型： &core::result::Result<usize, diesel::result::Error>
  
    let delete_count = result.unwrap_or(0);
    delete_count
}


