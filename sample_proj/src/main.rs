use crate::utils::establish_connection;

use diesel::deserialize::QueryableByName;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_query;

use chrono::{NaiveDateTime};

mod utils;

type DB = diesel::mysql::Mysql;

#[derive(Debug)]
pub struct Memos {
        id: i32,
        name: String,
        comment: String,
        //time: String,
        //time: DateTime<Utc>,
        time: NaiveDateTime,

}

impl QueryableByName<DB> for Memos {
        fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(row: &R,) -> diesel::deserialize::Result<Self> {
            Ok(Memos {
                         id:      row.get("id")?,
                         name:    row.get("name")?,
                         comment: row.get("comment")?,
                         time:    row.get::<diesel::mysql::types::Datetime, _>("time")?,
                     }
              )
        }
}

fn simple_sql() {

        let connection: MysqlConnection = establish_connection();

        let memos: Vec<Memos> = sql_query("SELECT id, name, comment, time FROM memos",).load(&connection).unwrap();

        for uu in memos.iter(){
                println!("{}\t{}\t{}\t{}", uu.id, uu.name, uu.comment, uu.time);
                //println!("{}\t{}\t{}", uu.id, uu.name, uu.comment);
        }
}

fn main (){
    eprintln! ("*** 開始 ***");
    simple_sql();
    eprintln! ("*** 終了 ***");
}
