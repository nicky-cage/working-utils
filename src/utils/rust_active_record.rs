#![allow(dead_code)]
use crate::common::database::get_pgsql_conn_string;
use crate::common::database::QueryBuilder;
use inflector::Inflector;
use sqlx::FromRow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::{Write, Read};
use log::{error,debug, info};
use std::path::Path;

const SAVE_PATH: &'static str = "/Users/ai/work/working-utils/src/active_records";
const TPL_CONTENT: &'static str = include_str!("tpl/rust_active_record.tpl");
const TPL_FIND_BY: &'static str = include_str!("tpl/rust_active_record_by.tpl");
const DB_NAME: &'static str = "working_database";

#[derive(Debug, FromRow)]
struct FieldInfo {
    pub field_name: String,
    pub field_type: String,
    pub comment: Option<String>,
    pub not_null: bool,
}

pub async fn create_file(table_name: &str) {
    let conn_string = get_pgsql_conn_string(DB_NAME);
    debug!("数据连接: {}", conn_string);

    let pool = pool_pgsql!(conn_string);

    let query = QueryBuilder::table("pg_class as c, pg_attribute as a ")
        .select(
            "col_description(a.attrelid,a.attnum) as comment, 
            format_type(a.atttypid,a.atttypmod) as field_type,
            a.attname as field_name, 
            a.attnotnull as not_null",
        )
        .r#where(" c.relname = '{TABLE_NAME}' and a.attrelid = c.oid and a.attnum > 0")
        .sql();
    let sql = query.replace("{TABLE_NAME}", table_name);

    // Vec<(字段名称, 类型, 注释)>
    let fields = match sqlx::query_as::<_, FieldInfo>(&sql).fetch_all(&pool).await {
        Ok(v) => v,
        Err(err) => {
            panic!("{}", err);
        }
    };
    let mut field_types: HashMap<&String, &'static str> = HashMap::new();
    for r in &fields { 
        if r.field_type.contains("character varying") {
            field_types.insert(&r.field_name, "String");
        } else if r.field_type.contains("bigint") { 
            field_types.insert(&r.field_name, "i64");
        } else if r.field_type.contains("int") {
            field_types.insert(&r.field_name, "i32");
        } else if r.field_type.contains("smallint") {
            field_types.insert(&r.field_name, "i8");
        } else if r.field_type.contains("decimal") {
            field_types.insert(&r.field_name, "f64");
        } else if r.field_type.contains("uuid") {
            field_types.insert(&r.field_name, "String");
        } else {
            field_types.insert(&r.field_name, "String");
        }
    }

    println!("hashmap: {:?}", field_types);

    let struct_name = table_name.to_singular().to_pascal_case(); // 结构体名称
    let mut struct_fields = String::new();
    let mut table_fields = String::from("id");
    let mut find_by_methods = String::new();
    let mut set_field_values = String::new();

    let mut index = 0;
    let field_count = fields.len();
    for r in &fields {
        let Some(field_type) = field_types.get(&r.field_name) else { 
            panic!("获取类型有误: {} -> {}", r.field_name, r.field_type);
        };
        struct_fields.push_str(&format!("pub {}: {},\n\t", r.field_name, field_type));
        let field_methods = TPL_FIND_BY
            .replace("{StructName}", &struct_name)
            .replace("{FieldName}", &r.field_name)
            .replace("{FieldType}", field_type)
            .replace("{TableName}", &table_name);
        find_by_methods.push_str(&field_methods);
        find_by_methods.push_str("\n\n");

        if r.field_name == "id" {
            continue;
        }

        if index == 0 { 
            set_field_values.push_str(&format!("db.set(\"{}\", &self.{})\n", r.field_name, r.field_name));
        } else if index == field_count - 2 { 
            set_field_values.push_str(&format!("\t\t\t.set(\"{}\", &self.{});", r.field_name, r.field_name));
        } else { 
            set_field_values.push_str(&format!("\t\t\t.set(\"{}\", &self.{})\n", r.field_name, r.field_name));
        }

        table_fields.push_str(&format!(", {}", r.field_name));
        index = index + 1;
    }

    let content = TPL_CONTENT
        .replace("{TableName}", table_name)
        .replace("{StructName}", &struct_name)
        .replace("{StructFields}", &struct_fields.trim())
        .replace("{TableFields}", &table_fields)
        .replace("{SetFieldValues}", &set_field_values)
        .replace("{FindByMethods}", &find_by_methods.trim());

    let save_path = format!("{}/{}.rs", SAVE_PATH, table_name);
    let saving_path = Path::new(&save_path);
    if saving_path.exists() { // 如果文件存在 - 则删除之
        let Ok(_) = std::fs::remove_file(&saving_path) else { 
            panic!("删除文件出错: {}", save_path);
        };
    }
    let Ok(mut file_writer) = OpenOptions::new().create(true).write(true).open(&save_path) else { 
        panic!("打开或者创建文件失败");
    };
    match file_writer.write_all(&content.as_bytes()) { 
        Ok(_) => { 
            info!("成功写入文件 {}.", &save_path);
        } 
        Err(err) => { 
            error!("写入文件失败 {}", err);
        }
    };

    // 写入文件: mod.rs
    let saving_mod_file_name = format!("{}/mod.rs", SAVE_PATH);
    let Ok(mut saving_mod_file) = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open(&saving_mod_file_name) else { 
        panic!("打开文件失败: {}", saving_mod_file_name);
    };
    let mut file_content = String::new();
    match saving_mod_file.read_to_string(&mut file_content) { 
        Ok(v) => { debug!("文件大小 {} bytes", v); }
        Err(err) => { panic!("读取文件发生错误: {}", err);}
    };
    let line_mod = format!("mod {};", table_name);
    if file_content.contains(&line_mod) { // 表示已经写过了
        return;
    }

    // 如果文件 mod.rs 未被加入, 则需写入文件
    let line_pub_use = format!("pub use {}::{};\n", table_name, &struct_name) ;
    let saving_content = format!("{}\n{}", line_mod, line_pub_use);
    match saving_mod_file.write_all(&saving_content.as_bytes()) { 
        Ok(_) => {
            info!("成功追加文件: {}", saving_mod_file_name);
        },
        Err(err) => { 
            error!("写入文件失败 {}", err);
        }
    };
}
