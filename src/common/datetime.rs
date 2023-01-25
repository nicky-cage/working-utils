#![allow(dead_code)]
use chrono::prelude::{DateTime, Local};
use chrono::Datelike;
use chrono::Timelike;

/// 秒数
#[inline]
pub fn timestamp() -> i64 {
    let now = now();
    now.timestamp()
}

/// 毫秒
#[inline]
pub fn timestamp_millis() -> i64 {
    let now = now();
    now.timestamp_millis()
}

/// 微秒
#[inline]
pub fn timestamp_micros() -> i64 {
    let now = now();
    now.timestamp_micros()
}

/// 当前的时间字符串
#[inline]
pub fn to_string() -> String {
    let local = now();
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 格式化时间
#[inline]
pub fn format(format_str: &str) -> String {
    let local = now();
    local.format(format_str).to_string()
}

/// 得到当前的日期
#[inline]
pub fn now() -> DateTime<Local> {
    Local::now()
}

/// 得到时分秒
#[inline]
pub fn time() -> (u32, u32, u32) {
    let now = now();
    (now.hour(), now.minute(), now.second())
}

/// 得到年月日
#[inline]
pub fn date() -> (u32, u32, u32) {
    let now = now();
    (now.year() as u32, now.month() as u32, now.day() as u32)
}
