extern crate async_std;
extern crate clap;

use clap::Parser;
use log::error;
use std::env;

mod active_records;
#[macro_use]
mod common;
mod consts;
mod utils;

/// working-utils 自定义源代码生成工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 指定框架: gin/actix-web
    #[arg(short, long, default_value = "actix-web")]
    framework: String,
    /// 创建类型: controller/model/active_record
    #[arg(short, long, default_value = "active_record")]
    create: String,
    /// 控制器/模型名称
    #[arg(short, long)]
    name: String,
}

#[async_std::main]
async fn main() {
    let args = Args::parse();

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    if args.framework == "actix-web" {
        if args.create == "controller" {
            utils::rust_controller::create_file().await;
        } else if args.create == "model" {
            utils::rust_model::create_file().await;
        } else if args.create == "active_record" {
            utils::rust_active_record::create_file(&args.name).await;
        }
    } else if args.framework == "gin" {
        if args.create == "controller" {
            utils::go_controller::create_file().await;
        } else if args.create == "model" {
            utils::go_model::create_file().await;
        } else if args.create == "active_record" {
            utils::go_active_record::create_file(&args.name).await;
        }
    } else {
        error!("未知框架: {}", &args.framework);
    }
}
