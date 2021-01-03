//mysql 链接地址
pub const MYSQL_URL: &'static str = "mysql://root:1234@localhost:3306/venus_web";
//server 地址
pub const SERVER_URL: &'static str = "127.0.0.1:8080";

/// 最多允许登录出错次数
pub const LOGIN_ERROR_MAX: usize = 1000;

/// 登录失败后锁定时间
pub const LOGIN_LOCKED_TIME: usize = 3600;