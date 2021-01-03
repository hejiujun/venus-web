use rbatis::crud::CRUD;
use rbatis::core::Result;
use rbatis::plugin::page::{Page, PageRequest};
use crate::dao::RB;
use crate::domain::SysUser;
use crate::dto::{UserPageDTO, LoginUser};

///用户信息分页查询
pub async fn user_page(arg: &UserPageDTO) -> Result<Page<SysUser>> {
    let wrapper = RB.new_wrapper()
        .do_if(arg.login_name.is_some(), |w| w.eq("login_name", &arg.login_name))
        .check()?;
    let mut result: Page<SysUser> = RB.fetch_page_by_wrapper("", &wrapper, &PageRequest::new(arg.page.unwrap_or(1), arg.size.unwrap_or(10))).await?;
    return Ok(result);
}

///用户信息用户名查询
pub async fn find_by_login_name(login_name: &str) -> Result<Option<SysUser>> {
    let wrapper = RB.new_wrapper()
        .eq("login_name", login_name)
        .check()?;
    return RB.fetch_by_wrapper("", &wrapper).await;
}