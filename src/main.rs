#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis_macro_driver;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_json;


use actix_web::{middleware, web, App, HttpServer};
use actix_session::{CookieSession};
use actix_files::Files;
use handlebars::Handlebars;
use rbatis::core::db::DBPoolOptions;
use time::Duration;
use std::io;

mod controller;
mod common;
mod config;
mod dao;
mod domain;
mod service;
mod dto;

use controller::{login_controller, index_controller, user_controller};
use config::MYSQL_URL;
use config::SERVER_URL;
use dao::RB;



#[actix_web::main]
async fn main() -> io::Result<()> {
    //日志追加器
    fast_log::init_log("log/venus.log", 1000, log::Level::Info, None, true);
    //链接数据库
    let mut opt = DBPoolOptions::new();
    opt.max_connections = 20;
    RB.link_opt(MYSQL_URL, &opt).await.unwrap();
    //模版引擎
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    HttpServer::new(move || {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(common::errors::error_handlers())
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "public/static/")) //静态文件目录
            .service(index_controller::index)
            .service(user_controller::page)
            .service(login_controller::login_html)
            .service(login_controller::captcha_img)
            .service(login_controller::login_in)
            .service(login_controller::logout)
    })
        .bind(SERVER_URL)?
        .run()
        .await
}






