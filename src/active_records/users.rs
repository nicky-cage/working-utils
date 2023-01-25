#![allow(dead_code)]
#![allow(unused)]

use crate::common::database::DB;
use crate::common::database::{Pool, QueryBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize, Default)]
pub struct User {
    pub id: String,
	pub username: String,
	pub password: String,
	pub fund_password: String,
	pub login_count: i32,
	pub login_last_at: i64,
	pub created: i64,
	pub updated: i64, 
}

impl User {
    /// 数据表名
    #[inline]
    pub fn table_name() -> &'static str {
        "users"
    }

    /// 字段名称
    #[inline]
    pub fn fields() -> &'static str {
        "id, username, password, fund_password, login_count, login_last_at, created, updated"
    }

    /// 新的实例
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// 依据条件/分页查找记录
    pub async fn find(pool: &Pool, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where(query.where_cond);
        for i in &query.where_binds {
            db.bind(i);
        }
        db.find::<User>(pool).await
    }

    /// 依据条件查找所有记录
    pub async fn find_all(pool: &Pool, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where(query.where_cond);
        for i in &query.where_binds {
            db.bind(i);
        }
        db.find::<User>(pool).await
    }

    /// 依据条件查找单条记录
    pub async fn get(pool: &Pool, query: &QueryBuilder) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where(query.where_cond).limit(1);
        for i in &query.where_binds {
            db.bind(i);
        }
        db.get::<User>(pool).await
    }

    // create
    pub async fn create(&self, pool: &Pool) -> u64 {
        let mut db = DB::table("users");
        db.set("username", &self.username)
			.set("password", &self.password)
			.set("fund_password", &self.fund_password)
			.set("login_count", &self.login_count)
			.set("login_last_at", &self.login_last_at)
			.set("created", &self.created)
			.set("updated", &self.updated);
        db.insert(pool).await
    }

    // update
    pub async fn update(&self, pool: &Pool) -> u64 {
        let mut db = DB::table("users");
        db.r#where("id = $1");
        db.bind(&self.id);
        db.set("username", &self.username)
			.set("password", &self.password)
			.set("fund_password", &self.fund_password)
			.set("login_count", &self.login_count)
			.set("login_last_at", &self.login_last_at)
			.set("created", &self.created)
			.set("updated", &self.updated);
        db.update(pool).await
    }

    // delete
    pub async fn delete(&self, pool: &Pool) -> u64 {
        let mut db = DB::table("users");
        db.r#where("id = $1");
        db.bind(&self.id);
        db.execute(pool).await
    }

    /// 依据字段获取分页记录 - find_by_id
    pub async fn find_by_id(&self, pool: &Pool, val: &String, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("id = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_id
    pub async fn find_all_by_id(&self, pool: &Pool, val: &String) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("id = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_id
    pub async fn get_by_id(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("id = $1")
            .bind(&self.id)
            .limit(1);
        db.get::<User>(pool).await
    }

    /// 依据字段获取分页记录 - find_by_username
    pub async fn find_by_username(&self, pool: &Pool, val: &String, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("username = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_username
    pub async fn find_all_by_username(&self, pool: &Pool, val: &String) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("username = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_username
    pub async fn get_by_username(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("username = $1")
            .bind(&self.username)
            .limit(1);
        db.get::<User>(pool).await
    }

    /// 依据字段获取分页记录 - find_by_password
    pub async fn find_by_password(&self, pool: &Pool, val: &String, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("password = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_password
    pub async fn find_all_by_password(&self, pool: &Pool, val: &String) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("password = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_password
    pub async fn get_by_password(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("password = $1")
            .bind(&self.password)
            .limit(1);
        db.get::<User>(pool).await
    }

    /// 依据字段获取分页记录 - find_by_fund_password
    pub async fn find_by_fund_password(&self, pool: &Pool, val: &String, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("fund_password = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_fund_password
    pub async fn find_all_by_fund_password(&self, pool: &Pool, val: &String) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("fund_password = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_fund_password
    pub async fn get_by_fund_password(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("fund_password = $1")
            .bind(&self.fund_password)
            .limit(1);
        db.get::<User>(pool).await
    }

    /// 依据字段获取分页记录 - find_by_login_count
    pub async fn find_by_login_count(&self, pool: &Pool, val: &i32, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("login_count = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_login_count
    pub async fn find_all_by_login_count(&self, pool: &Pool, val: &i32) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("login_count = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_login_count
    pub async fn get_by_login_count(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("login_count = $1")
            .bind(&self.login_count)
            .limit(1);
        db.get::<User>(pool).await
    }

    /// 依据字段获取分页记录 - find_by_login_last_at
    pub async fn find_by_login_last_at(&self, pool: &Pool, val: &i64, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("login_last_at = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_login_last_at
    pub async fn find_all_by_login_last_at(&self, pool: &Pool, val: &i64) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("login_last_at = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_login_last_at
    pub async fn get_by_login_last_at(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("login_last_at = $1")
            .bind(&self.login_last_at)
            .limit(1);
        db.get::<User>(pool).await
    }

    /// 依据字段获取分页记录 - find_by_created
    pub async fn find_by_created(&self, pool: &Pool, val: &i64, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("created = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_created
    pub async fn find_all_by_created(&self, pool: &Pool, val: &i64) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("created = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_created
    pub async fn get_by_created(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("created = $1")
            .bind(&self.created)
            .limit(1);
        db.get::<User>(pool).await
    }

    /// 依据字段获取分页记录 - find_by_updated
    pub async fn find_by_updated(&self, pool: &Pool, val: &i64, query: &QueryBuilder) -> Vec<User> {
        let mut db = DB::table("users");
        db.limit(query.limit)
            .offset(query.offset)
            .r#where("updated = $1")
            .bind(val);
        db.find::<User>(pool).await
    }

    /// 依据字段获取所有记录 - find_all_by_updated
    pub async fn find_all_by_updated(&self, pool: &Pool, val: &i64) -> Vec<User> {
        let mut db = DB::table("users");
        db.r#where("updated = $1").bind(val);
        db.find_all::<User>(pool).await
    }

    /// 依据字段获取单条记录 - get_by_updated
    pub async fn get_by_updated(&self, pool: &Pool) -> Option<User> {
        let mut db = DB::table("users");
        db.r#where("updated = $1")
            .bind(&self.updated)
            .limit(1);
        db.get::<User>(pool).await
    } 
}