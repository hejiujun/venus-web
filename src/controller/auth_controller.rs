use actix_web::{HttpResponse, web};
use actix_session::{Session};
use handlebars::Handlebars;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use crate::service::user_service;
use crate::config::LOGIN_ERROR_MAX;
use crate::config::LOGIN_LOCKED_TIME;
use crate::dto::{RespVO, LoginUser};
use crate::common::utils;
use crate::common::auth;
use std::ptr::null;


#[get("/login")]
pub async fn login_html(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "name": "金星"
    });
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}

/// 用户登录
#[post("/login")]
pub async fn login(session: Session, arg: web::Json<LoginUser>) -> HttpResponse {
    //如果session中记录的有锁定时间
    if let Ok(locked_time) = session.get::<usize>("locked_time") {
        if let Some(n) = locked_time {
            if (NaiveDateTime::now().timestamp() as usize) - n < LOGIN_LOCKED_TIME {
                return RespVO::<u64>::from_error_info("", "登录次败次数过多,请2小时后再次尝试").resp();
            }
        }
    }

    let mut failure_count = 0_usize; //登录失败次数
    if let Ok(failure) = session.get::<usize>("failure_count") {  //检测登录失败次数
        if let Some(n) = failure {
            failure_count = n; //已经失败的次数
            if n > LOGIN_ERROR_MAX {
                if let Err(message) = session.set::<usize>("locked_time", NaiveDateTime::now().timestamp() as usize) {
                    return RespVO::<u64>::from_error_info("", &message.to_string()).resp();
                }
                return RespVO::<u64>::from_error_info("", "失败次数过多, 请稍后重试").resp();
            }
        }
    } else {
        if let Err(message) = session.set::<usize>("failure_count", failure_count) {
            return RespVO::<u64>::from_error_info("", &message.to_string()).resp();
        }
    } //设置登录失败次数的默认值

    let user = user_service::find_by_login_name(&arg.login_name.as_ref().unwrap()).await;
    if let Ok(matching) = utils::verify(&user.clone().unwrap().unwrap().password.unwrap(), &arg.password.as_ref().unwrap()) {
        session.remove("failure_count"); //清空失败次数
        session.remove("locked_time"); //清空锁定时间
        session.set::<String>("user_id", user.clone().unwrap().unwrap().user_id.unwrap()).unwrap(); //session
        session.set::<String>("user_name", user.clone().unwrap().unwrap().login_name.unwrap()).unwrap(); //session
        //session.set::<String>("role_id", role_id).unwrap(); //session
        return RespVO::from_result(&user).resp();
    } else {
        session.set::<usize>("failure_count", failure_count + 1).unwrap();
        return RespVO::<u64>::from_error_info("", "用户名称或密码错误").resp();
    }
}

#[get("/logout")]
pub async fn logout(hb: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    session.remove("user_id");
    session.remove("user_name");
    session.remove("role_id");
    let data = json!({
        "name": "金星"
    });
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}