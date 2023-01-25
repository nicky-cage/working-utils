#![allow(dead_code)]
#![allow(unused)]

use crate::common::database::DB;
use crate::common::database::{Pool, QueryBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct {StructName} {
    {StructFields} 
}

impl {StructName} {
    /// 数据表名
    #[inline]
    pub fn table_name() -> &'static str {
        "{TableName}"
    }

    /// 字段名称
    #[inline]
    pub fn fields() -> &'static str {
        "{TableFields}"
    }

    /// 新的实例
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 依据条件/分页查找记录
    pub async fn find(pool: &Pool, query: &QueryBuilder) -> Vec<{StructName}> {
        let mut db = DB::table("{TableName}");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where(query.where_cond);
        for i in &query.where_binds {
            db.bind(i);
        }
        db.find::<{StructName}>(pool).await
    }

    /// 依据条件查找所有记录
    pub async fn find_all(pool: &Pool, query: &QueryBuilder) -> Vec<{StructName}> {
        let mut db = DB::table("{TableName}");
        db.r#where(query.where_cond);
        for i in &query.where_binds {
            db.bind(i);
        }
        db.find::<{StructName}>(pool).await
    }

    /// 依据条件查找单条记录
    pub async fn get(pool: &Pool, query: &QueryBuilder) -> Option<{StructName}> {
        let mut db = DB::table("{TableName}");
        db.r#where(query.where_cond).limit(1);
        for i in &query.where_binds {
            db.bind(i);
        }
        db.get::<{StructName}>(pool).await
    }

    // create
    pub async fn create(&self, pool: &Pool) -> u64 {
        let mut db = DB::table("{TableName}");
        {SetFieldValues}
        db.insert(pool).await
    }

    // update
    pub async fn update(&self, pool: &Pool) -> u64 {
        let mut db = DB::table("{TableName}");
        db.r#where("id = $1");
        db.bind(&self.id);
        {SetFieldValues}
        db.update(pool).await
    }

    // delete
    pub async fn delete(&self, pool: &Pool) -> u64 {
        let mut db = DB::table("{TableName}");
        db.r#where("id = $1");
        db.bind(&self.id);
        db.execute(pool).await
    }

    {FindByMethods} 
}