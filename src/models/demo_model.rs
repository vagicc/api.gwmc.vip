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
            .get_result::<Demo>(&connection).unwrap();
        println!("插入数据返回：{:#?}", result);
        result
    }
}

pub fn get_list() {
    let connection = get_connection();
}
