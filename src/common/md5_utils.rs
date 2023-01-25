#![allow(dead_code)]

/// 安全密鑰
const SECRET_KEYS: &str = "!s@w4$qS%^(_123-=0Xha9452sLW^%sfa9)\\";

/// md5
#[inline]
pub fn str(content: &str) -> String {
    let encrypt = md5::compute(content);
    format!("{:x}", encrypt)
}
