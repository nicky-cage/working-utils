use crate::consts::{DB_HOST, DB_PASS, DB_USER, PG_HOST, PG_PASS, PG_USER};

pub mod db;
pub mod query_builder;

pub use db::DB;
pub use query_builder::QueryBuilder;

pub type Pool = sqlx::Pool<sqlx::Postgres>;
pub type DriverType = sqlx::Postgres;
pub type RowType = sqlx::postgres::PgRow;
pub type Transaction<'t> = sqlx::Transaction<'t, sqlx::Postgres>;
// pub type PgPoolOptions = sqlx::postgres::PgPoolOptions;
// pub type MyPoolOptions = sqlx::mysql::MySqlPoolOptions;

/// 获取数据库连接
pub fn get_mysql_conn_string(db_name: &'static str) -> String {
    format!("mysql://{}:{}@{}/{}", DB_USER, DB_PASS, DB_HOST, db_name)
}

/// 获取数据库连接
pub fn get_pgsql_conn_string(db_name: &'static str) -> String {
    format!("postgres://{}:{}@{}/{}", PG_USER, PG_PASS, PG_HOST, db_name)
}

#[macro_export]
macro_rules! pool_mysql {
    ($conn_string: expr) => {
        match sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&$conn_string)
            .await
        {
            Ok(v) => v,
            Err(err) => {
                panic!("err: {}", err);
            }
        }
    };
}

#[macro_export]
macro_rules! pool_pgsql {
    ($conn_string: expr) => {
        match sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&$conn_string)
            .await
        {
            Ok(v) => v,
            Err(err) => {
                panic!("err: {}", err);
            }
        }
    };
}
