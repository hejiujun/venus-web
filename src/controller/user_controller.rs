use actix_web::{HttpResponse,Responder, web};
use handlebars::Handlebars;
use crate::service::user_service;
use crate::dto::{UserPageDTO,RespVO};
use actix_web::web::Json;
use crate::domain::SysUser;

#[post("/user")]
pub async fn page(arg:web::Json<UserPageDTO>) -> HttpResponse{


    let mut result = user_service::user_page(&arg.0).await;
    // let data = json!({
    //     "data":"result"
    //
    // });
    // if arg.is_ok() {
    //     Self {
    //         code: Some("SUCCESS".to_string()),
    //         msg: None,
    //         data: arg.clone().ok(),
    //     }
    // } else {
    //     Self {
    //         code: Some("FAIL".to_string()),
    //         msg: Some(arg.clone().err().unwrap().to_string()),
    //         data: None,
    //     }
    // }
//    let body = hb.render("user", &data).unwrap();
  //  HttpResponse::Ok().body(body)
     return RespVO::from_result(&result).resp();
   /* let data = json!({
        "data": result
    });
    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)*/

}

/*pub async fn save(arg:web::Json<SysUser>)-> HttpResponse{

}*/

