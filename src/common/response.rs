#![allow(dead_code)]
use actix_web::HttpResponse;
use serde::Serialize;

/// 处理成功
#[derive(Serialize)]
pub struct JsonOK {
    pub code: u16,
}

/// 处理失败
#[derive(Serialize)]
pub struct JsonError<T: AsRef<str>> {
    pub code: u16,
    pub message: T,
}

/// 返回结果
#[derive(Serialize)]
pub struct JsonResult<T: Serialize> {
    pub code: u16,
    pub data: T,
}

/// 授权
#[derive(Serialize)]
pub struct JsonAuth {
    pub authorization: String,
}

/// 返回200
pub fn ok() -> HttpResponse {
    HttpResponse::Ok().json(JsonOK { code: 0 })
}

/// 拒絕訪問
pub fn deny() -> HttpResponse {
    HttpResponse::Ok().body("deny")
}

/// 返回错误消息
pub fn error<T: AsRef<str>>(message: T) -> HttpResponse {
    let err = JsonError {
        code: 500,
        message: message.as_ref(),
    };
    let result = serde_json::to_string(&err).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(result)
}

/// 返回数据
pub fn result<T: Serialize>(result: &T) -> HttpResponse {
    let res = JsonResult {
        code: 0,
        data: result,
    };
    HttpResponse::Ok().json(res)
}

// 页面重定向
// pub fn redirect(url: &str) -> HttpResponse {
//     HttpResponse::Found().header(header::LOCATION, url).finish()
// }
