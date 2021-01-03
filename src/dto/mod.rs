use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rbatis::core::Error;
use actix_http::Response;
use actix_web::HttpResponse;

/// 用户分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page: Option<u64>,
    pub size: Option<u64>,
    pub login_name: Option<String>,
}

/// 用户登录
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginUser {
    pub login_name: Option<String>,
    pub password: Option<String>,
}

///请求返回
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RespVO<T> {
    pub msg: Option<String>,
    pub code: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T> where T: Serialize + DeserializeOwned + Clone {
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some("SUCCESS".to_string()),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some("FAIL".to_string()),
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some("SUCCESS".to_string()),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(code: &str, arg: &Error) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = "FAIL".to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: &str, info: &str) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = "FAIL".to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn resp(&self) -> Response {
        return HttpResponse::Ok().content_type("json").body(self.to_string());
    }
}

impl<T> ToString for RespVO<T> where T: Serialize + DeserializeOwned + Clone {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}