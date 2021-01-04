use actix_web::{HttpResponse, web};
use actix_session::{Session};
use handlebars::Handlebars;
use crate::common::auth::Auth;

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
pub async fn index(hb: web::Data<Handlebars<'_>>, session: Session) -> HttpResponse {
    if !Auth::check_login(&session) {
        let data = json!({
        "error": "请登录后访问！"
        });
        let body = hb.render("login", &data).unwrap();
        return HttpResponse::Ok().body(body);
    }
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    return HttpResponse::Ok().body(body);
}