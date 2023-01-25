#![allow(dead_code)]

use super::{DriverType, Pool, RowType, Transaction};
use futures::future::BoxFuture;
use log::error;

pub struct DB {
    /// 表名
    pub table_name: &'static str,
    /// 要执行select的字段
    pub select_fields: &'static str,
    /// limit
    pub page_limit: i32,
    /// offset
    pub page_offset: i32,
    /// 是否设置limit
    set_limit: bool,
    /// 是否设置offset
    set_offset: bool,
    /// 默认字段 - 用于set()
    fields: Vec<&'static str>,
    /// 默认字段的值 - 用于set()
    values: Vec<String>,
    /// 默认条件 - 用于 r#where()
    where_conds: &'static str,
    /// 查询语句 - 用于 prepare_sql()
    prepare_sql: &'static str,
    /// 绑定的值 - 用于bind()
    bind_values: Vec<String>,
}

impl DB {
    /// 生成依赖于表的DB的实例
    pub fn table(table_name: &'static str) -> Self {
        Self {
            select_fields: "*",
            table_name: table_name,
            page_limit: 15,
            page_offset: 0,
            fields: vec![],
            values: vec![],
            where_conds: "",
            set_limit: false,
            set_offset: false,
            prepare_sql: "",
            bind_values: vec![],
        }
    }

    /// 选择部分字段
    pub fn select(&mut self, fields: &'static str) -> &mut Self {
        self.select_fields = fields;
        self
    }

    /// 设备Where条件
    pub fn r#where(&mut self, where_cond: &'static str) -> &mut Self {
        self.where_conds = where_cond;
        self
    }

    /// 设置limit
    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.page_limit = limit;
        self.set_limit = true;
        self
    }

    /// 设置offset
    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.page_offset = offset;
        self.set_offset = true;
        self
    }

    /// 设置相关字段的值
    pub fn set<T: ToString>(&mut self, field: &'static str, value: &T) -> &mut Self {
        let value_str = value.to_string();
        if value_str == "" {
            return self;
        }
        self.fields.push(field);
        self.values.push(value_str);
        self
    }

    /// 需要先调用 set() 方法, 设置各个字段的值
    pub async fn insert(&mut self, pool: &Pool) -> u64 {
        let fields = self.fields.join(",").to_string();
        let mut values_quotes: Vec<&'static str> = vec![];
        for _ in &self.fields {
            values_quotes.push("?");
        }
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ()",
            fields,
            values_quotes.join("")
        );
        let mut executor = sqlx::query::<DriverType>(&sql);
        for r in &self.values {
            executor = executor.bind(r);
        }
        match executor.execute(pool).await {
            Ok(v) => v.rows_affected(),
            Err(err) => {
                println!("执行数据发生错误:{}", err.to_string());
                0
            }
        }
    }

    /// 更橷数据库值
    pub async fn update(&mut self, pool: &Pool) -> u64 {
        let mut update_arr: Vec<String> = vec![];
        for r in &self.fields {
            update_arr.push(format!("{} = ?", r))
        }
        let sql = format!(
            "UPDATE {} SET {} WHERE {}",
            self.table_name,
            update_arr.join(","),
            self.where_conds,
        );
        let mut executor = sqlx::query::<DriverType>(&sql);
        for r in &self.values {
            executor = executor.bind(r);
        }
        for r in &self.bind_values {
            executor = executor.bind(r);
        }
        match executor.execute(pool).await {
            Ok(v) => v.rows_affected(),
            Err(err) => {
                println!("执行数据发生错误:{}", err.to_string());
                0
            }
        }
    }

    // 查找数据列表
    pub async fn find<T>(&self, pool: &Pool) -> Vec<T>
    where
        T: for<'r> sqlx::FromRow<'r, RowType> + std::marker::Send + Unpin,
    {
        let where_str = self.where_conds;
        let sql = format!(
            "SELECT {} FROM {} WHERE {} {} {}",
            if self.select_fields == "" {
                "*"
            } else {
                self.select_fields
            },
            self.table_name,
            where_str,
            if self.set_limit {
                format!("LIMIT {}", self.page_limit)
            } else {
                "".to_owned()
            },
            if self.set_offset {
                format!("OFFSET {}", self.page_offset)
            } else {
                "".to_owned()
            }
        );
        let mut query = sqlx::query_as::<DriverType, T>(&sql);
        for r in &self.bind_values {
            query = query.bind(r);
        }
        match query.fetch_all(pool).await {
            Ok(rows) => rows,
            Err(err) => {
                println!("获取数据发生错误: {}", err);
                vec![]
            }
        }
    }

    // 查找数据列表
    pub async fn find_all<T>(&self, pool: &Pool) -> Vec<T>
    where
        T: for<'r> sqlx::FromRow<'r, RowType> + std::marker::Send + Unpin,
    {
        let where_str = self.where_conds;
        let sql = format!(
            "SELECT {} FROM {} WHERE {}",
            if self.select_fields == "" {
                "*"
            } else {
                self.select_fields
            },
            self.table_name,
            where_str,
        );
        let mut query = sqlx::query_as::<DriverType, T>(&sql);
        for r in &self.bind_values {
            query = query.bind(r);
        }
        match query.fetch_all(pool).await {
            Ok(rows) => rows,
            Err(err) => {
                println!("获取数据发生错误: {}", err);
                vec![]
            }
        }
    }

    /// 获取单条记录
    pub async fn get<T>(&self, pool: &Pool) -> Option<T>
    where
        T: for<'r> sqlx::FromRow<'r, RowType> + std::marker::Send + Unpin,
    {
        let where_str = self.where_conds;
        let sql = format!(
            "SELECT {} FROM {} WHERE {} LIMIT 1",
            if self.select_fields == "" {
                "*"
            } else {
                self.select_fields
            },
            self.table_name,
            where_str
        );
        let mut query = sqlx::query_as::<DriverType, T>(&sql);
        for r in &self.bind_values {
            query = query.bind(r);
        }
        match query.fetch_one(pool).await {
            Ok(row) => Some(row),
            Err(err) => {
                println!("获取数据发生错误: {}", err);
                None
            }
        }
    }

    /// 随操作的语句, delete, execute 用到
    pub fn prepare(&mut self, prepare_sql: &'static str) -> &mut Self {
        self.prepare_sql = prepare_sql;
        self
    }

    /// 绑定查询的值, delete, select, execute 用到
    pub fn bind<T: ToString>(&mut self, arg: &T) -> &mut Self {
        self.bind_values.push(arg.to_string());
        self
    }

    /// 执行语句
    pub async fn execute(&self, pool: &Pool) -> u64 {
        let mut executor = sqlx::query::<DriverType>(&self.prepare_sql);
        for r in &self.bind_values {
            executor = executor.bind(r);
        }

        match executor.execute(pool).await {
            Ok(v) => v.rows_affected(),
            Err(err) => {
                println!("执行语句发生错误 {}", err);
                0
            }
        }
    }

    /// 执行事务 - 版本2
    pub async fn transaction_v2<'a, F>(pool: &Pool, callback: F) -> Result<(), &'static str>
    where
        F: 'a
            + for<'c> FnOnce(
                &'c mut sqlx::Transaction<'_, DriverType>,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<(), &'static str>> + Send + 'c>,
            >
            + Send
            + Sync,
    {
        let mut transaction = if let Ok(v) = pool.begin().await {
            v
        } else {
            return Err("执行BEGIN发生错误");
        };

        if let Err(err) = callback(&mut transaction).await {
            error!("{err}");
            if let Err(rerr) = transaction.rollback().await {
                error!("{rerr}");
                return Err("执行ROLLBACK发生错误");
            }
            return Err(err);
        }

        if let Err(err) = transaction.commit().await {
            error!("{err}");
            return Err("执行COMMIT时出错");
        }

        Ok(())
    }

    /// 执行事务
    pub async fn transaction<'a, F>(pool: &Pool, callback: F) -> Result<(), &'static str>
    where
        for<'c> F:
            FnOnce(&'c mut Transaction) -> BoxFuture<'c, Result<(), &'static str>> + Send + Sync,
    {
        let mut transaction = if let Ok(v) = pool.begin().await {
            v
        } else {
            return Err("执行BEGIN发生错误");
        };

        if let Err(err) = callback(&mut transaction).await {
            error!("{err}");
            if let Err(rerr) = transaction.rollback().await {
                error!("{rerr}");
                return Err("执行ROLLBACK发生错误");
            }
            return Err(err);
        }

        if let Err(err) = transaction.commit().await {
            error!("{err}");
            return Err("执行COMMIT时出错");
        }

        Ok(())
    }
}
