use actix_session::{Session};
use actix_web::{HttpRequest};
pub struct Auth {}

impl Auth {
    /// 登录检测
    pub fn check_login(session: &Session) -> bool {
        if let Ok(v) = session.get::<String>("user_id") {
            if let Some(n) = v {
                if !n.is_empty() {
                    return true;
                }
            }
        }
        false
    }

    // 检测路由权限
    /*pub fn check_auth(request: &HttpRequest, session: &Session) -> bool {
    if let Ok(v) = session.get::<String>("role_id") { //角色编号
        if let Some(role_id) = v {
            let path = request.path();
            return caches::admin_roles::allow_access(role_id, path);
        }
    }
    false
}*/
}