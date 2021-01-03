use rbatis::rbatis::Rbatis;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;


pub mod mapper;

// Rbatis初始化
lazy_static! {
  pub static ref RB:Rbatis={
     let mut rbatis = Rbatis::new();
     //logic plugin 设置逻辑删除插件
     rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt("del_flag",1,0)));
     return rbatis;
  };
}