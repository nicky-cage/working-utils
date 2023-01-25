#![allow(dead_code)]

/// 生成订单号
pub fn create_order() -> String {
    let dt = super::datetime::now();
    let mut s = String::from("NU");
    s.push_str(&dt.format("%Y%m%d%H%M%S").to_string());
    s.push_str(&super::random::rand_str(4));
    s
}
