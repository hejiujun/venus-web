use chrono::NaiveDateTime;


//用户表
#[crud_enable]
#[derive(Clone, Debug)]
pub struct SysUser {
    pub user_id: Option<String>,
    pub dept_id: Option<String>,
    pub login_name: Option<String>,
    pub user_name: Option<String>,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub phonenumber: Option<String>,
    pub sex: Option<String>,
    pub avatar: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub status: Option<String>,
    pub del_flag: Option<String>,
    pub login_ip: Option<String>,
    pub login_date: Option<NaiveDateTime>,
    pub create_by: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<NaiveDateTime>,
    pub remark: Option<String>,
}