# venus-web
基于Rust语言开发的web项目

#### 引用技术
- actix-web
- rbatis
- handlebars
- mysql
- redis
#### 快速使用教程
- 安装mysql，建数据库rust_test，将sql/venus_web.sql文件导入数据库
- 配置数据库连接src/config/mod.rs
  ```cmd
  pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/rust_test"
  ```
- 配置Redis连接src/config/mod.rs
  ```cmd
  pub const REDIS_URL:&'static str="redis://127.0.0.1:6379"
  ```
- 使用Clion克隆导入venus_web项目，命令行执行或者点开main.rs点击按钮运行
  ```cmd
  cargo run
  ```
- 访问http://127.0.0.1:8080/login 用户名：admin 密码：123456 登录后可以访问其他功能




