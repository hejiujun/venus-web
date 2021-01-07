use actix_web::{HttpResponse, web, HttpRequest};
use actix_session::{Session};
use handlebars::Handlebars;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;
use captcha::Captcha;
use captcha::filters::{Dots, Noise, Wave};

use crate::service::{user_service};
use crate::service::REDIS_SERVICE;
use crate::config::LOGIN_ERROR_MAX;
use crate::config::LOGIN_LOCKED_TIME;
use crate::dto::{RespVO, LoginUser, CatpchaDTO};
use crate::common::utils;
use crate::common::auth;


#[get("/login")]
pub async fn login_html(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "name": "金星"
    });
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}

// 用户登录
#[post("/login_in")]
pub async fn login_in(session: Session, arg: web::Json<LoginUser>) -> HttpResponse {
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

    //验证码校验
    let validate = REDIS_SERVICE.get_string("captch:account_118000").await;
    if validate.is_err() {
        let response = RespVO::from_result(&validate).resp();
        return response;
    }else{
      if !validate.unwrap().as_str().eq(arg.validate_code.as_ref().unwrap()){
          return RespVO::<u64>::from_error_info("", "验证码输入不正确").resp();
      }
    }

    //查询用户信息
    let user = user_service::find_by_login_name(&arg.login_name.as_ref().unwrap()).await;
    //校验密码
    let m = utils::verify(&user.clone().unwrap().unwrap().password.unwrap(), &arg.password.as_ref().unwrap());
    return if m {
        session.remove("failure_count"); //清空失败次数
        session.remove("locked_time"); //清空锁定时间
        session.set::<String>("user_id", user.clone().unwrap().unwrap().user_id.unwrap()).unwrap(); //session
        session.set::<String>("user_name", user.clone().unwrap().unwrap().login_name.unwrap()).unwrap(); //session
        //session.set::<String>("role_id", role_id).unwrap(); //session
        RespVO::from_result(&user).resp()
    } else {
        session.set::<usize>("failure_count", failure_count + 1).unwrap();
        RespVO::<u64>::from_error_info("", "用户名称或密码错误").resp()
    };
}

//退出登录
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

//验证码
#[get("/captchaImg/{data}")]
pub async fn captcha_img(web::Path(info): web::Path<(String)>) -> HttpResponse {
    let mut captcha = Captcha::new();
    captcha.add_chars(4)
        .apply_filter(Noise::new(0.1))
        .apply_filter(Wave::new(1.0, 10.0).horizontal())
        // .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(160, 60)
        .apply_filter(Dots::new(4));
    let png = captcha.as_png().unwrap();
    let captcha_str = captcha.chars_as_string().to_lowercase();
    println!("{},{}", &captcha_str.as_str(), &info.as_str());
    let result = REDIS_SERVICE.set_string("captch:account_118000", &captcha_str.as_str()).await;
    if result.is_err() {
        return RespVO::from_result(&result).resp();
    }
    HttpResponse::Ok().content_type("image/png").body(png)
}

