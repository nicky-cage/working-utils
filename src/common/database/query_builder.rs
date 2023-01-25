#![allow(dead_code)]

pub struct QueryBuilder {
    pub select_fields: &'static str,
    pub where_cond: &'static str,
    pub where_binds: Vec<String>,
    pub table_name: &'static str,
    pub limit: i32,
    pub offset: i32,
    pub fields: Vec<&'static str>,
    pub values: Vec<String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            select_fields: "*",
            where_cond: "",
            table_name: "",
            limit: 15,
            offset: 0,
            fields: vec![],
            values: vec![],
            where_binds: vec![],
        }
    }

    pub fn table(table_name: &'static str) -> Self {
        Self {
            select_fields: "*",
            where_cond: "",
            table_name,
            limit: 15,
            offset: 0,
            fields: vec![],
            values: vec![],
            where_binds: vec![],
        }
    }

    pub fn select(&mut self, fields: &'static str) -> &mut Self {
        self.select_fields = fields;
        self
    }

    pub fn r#where(&mut self, where_cond: &'static str) -> &mut Self {
        self.where_cond = where_cond;
        self
    }

    pub fn bind<T: ToString>(&mut self, where_bind: &T) -> &mut Self {
        self.where_binds.push(where_bind.to_string());
        self
    }

    pub fn set_limit(&mut self, limit: i32) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn set_offset(&mut self, offset: i32) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn sql(&self) -> String {
        format!(
            "SELECT {} FROM {} WHERE {}",
            if self.select_fields == "" {
                "*"
            } else {
                self.select_fields
            },
            self.table_name,
            if self.where_cond == "" {
                "1 = 1"
            } else {
                self.where_cond
            }
        )
    }

    pub fn sql_page(&self) -> String {
        format!(
            "SELECT {} FROM {} WHERE {} LIMIT {} OFFSET {}",
            if self.select_fields == "" {
                "*"
            } else {
                self.select_fields
            },
            self.table_name,
            if self.where_cond == "" {
                "1 = 1"
            } else {
                self.where_cond
            },
            if self.limit == 0 { 15 } else { self.limit },
            if self.offset == 0 { 0 } else { self.offset }
        )
    }
}
