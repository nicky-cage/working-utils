use crate::common::database::get_mysql_conn_string;
use inflector::Inflector;
use std::fs;
use std::path::Path;
use std::io::prelude::Write;

const SAVE_PATH: &'static str = "/Users/ai/work/dating-platform/models/src";
const TPL_CONTENT: &'static str = include_str!("tpl/go_active_record.tpl");
const DB_NAME: &'static str = "dating_paopao";

pub async fn create_file(table_name: &str) {
    let conn_string = get_mysql_conn_string(DB_NAME);
    println!("CONN STRING: {}", conn_string);

    let pool = pool_mysql!(conn_string);

    let sql_get_fields = concat!(
        "select COLUMN_NAME AS name, COLUMN_TYPE as type, COLUMN_COMMENT as comment ",
        "from information_schema.COLUMNS ",
        "where TABLE_NAME = '{TABLE_NAME}' AND TABLE_SCHEMA = '{DB_NAME}';"
    );
    let sql = sql_get_fields
        .replace("{TABLE_NAME}", &table_name)
        .replace("{DB_NAME}", DB_NAME);

    // Vec<(字段名称, 类型, 注释)>
    let rows: Vec<(String, String, String)> = match sqlx::query_as(&sql).fetch_all(&pool).await {
        Ok(v) => v,
        Err(err) => {
            panic!("{}", err);
        }
    };

    let content = String::from(TPL_CONTENT);
    let mut fields = String::new();
    // let mut fields: Vec<String> = Vec::new();
    for r in &rows {
        if r.0 == "id" {
            continue; // 跳过id
        }

        let row_name = r.0.to_pascal_case();
        let row_type = if r.0 == "created" || r.0 == "updated" || r.0 == "deleted" {
            "int64"
        } else {
            if r.1.contains("int") {
                "int"
            } else if r.1.contains("decimal") {
                "float64"
            } else {
                "string"
            }
        };
        let line = format!(
            " {} {} `json:\"{}\" xorm:\"{}\"` // {}\n",
            row_name, row_type, r.0, r.0, r.2
        );
        fields.push_str(&line);
    }

    let model_name = table_name.to_singular().to_pascal_case();
    let model_instance = table_name.to_pascal_case();

    let save_content = content
        .replace("{ModelFields}", &fields)
        .replace("{ModelName}", &model_name)
        .replace("{ModelNames}", &model_instance)
        .replace("{TableName}", table_name);

    println!("{}", save_content);
    if !Path::new(SAVE_PATH).exists() {
        panic!("路径不存在, 无法保存文件: {}", SAVE_PATH);
    }

    let file_path = format!("{}/{}.go", SAVE_PATH, model_instance);
    if Path::new(&file_path).exists() { 
        panic!("文件已经存在, 如果要写入, 请先手工删除");
    }

    let Ok(mut writer) =  fs::OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(&file_path) else { 
            panic!("创建文件失败");
        };

    match writer.write_all(&save_content.as_bytes()) { 
        Ok(_) => { 
            println!("成功写入文件: {}.", file_path)
        }, 
        Err(err) => {
            panic!("写入文件出错: {}", err);
        }
    };
}
