
-- ----------------------------
-- 1、部门表
-- ----------------------------
drop table if exists sys_dept;
create table sys_dept (
                          dept_id 			varchar(32) 		not null     comment '部门id',
                          parent_id 		varchar(32) 		default '' 			       comment '父部门id',
                          ancestors 		varchar(50)     default '' 			       comment '祖级列表',
                          dept_name 		varchar(30) 	default '' 				   comment '部门名称',
                          order_num 		int(4) 			default 0 			       comment '显示顺序',
                          leader            varchar(20)     default ''                 comment '负责人',
                          phone             varchar(11)     default ''                 comment '联系电话',
                          email             varchar(50)     default ''                 comment '邮箱',
                          status 			char(1) 		default '0' 			   comment '部门状态（0正常 1停用）',
                          del_flag			char(1) 		default '0' 			   comment '删除标志（0代表存在 2代表删除）',
                          create_by         varchar(64)     default ''                 comment '创建者',
                          create_time 	    datetime   NOT NULL ON UPDATE CURRENT_TIMESTAMP comment '创建时间',
                          update_by         varchar(64)     default ''                 comment '更新者',
                          update_time       datetime                                   comment '更新时间',
                          primary key (dept_id)
) engine=innodb comment = '部门表';


-- ----------------------------
-- 2、用户信息表
-- ----------------------------
drop table if exists sys_user;
create table sys_user (
                          user_id 			varchar(32) 		not null     comment '用户ID',
                          dept_id 			varchar(32) 		default ''			   comment '部门ID',
                          login_name 		varchar(30) 	not null 				   comment '登录账号',
                          user_name 		varchar(30) 	not null 				   comment '用户昵称',
                          user_type 		varchar(2) 	    default '00' 		       comment '用户类型（00系统用户）',
                          email  			varchar(50) 	default '' 				   comment '用户邮箱',
                          phonenumber  		varchar(11) 	default '' 				   comment '手机号码',
                          sex  		        char(1) 	    default '0' 			   comment '用户性别（0男 1女 2未知）',
                          avatar            varchar(100) 	default '' 				   comment '头像路径',
                          password 			varchar(50) 	default '' 				   comment '密码',
                          salt 				varchar(20) 	default '' 				   comment '盐加密',
                          status 			char(1) 		default '0' 			   comment '帐号状态（0正常 1停用）',
                          del_flag			char(1) 		default '0' 			   comment '删除标志（0代表存在 2代表删除）',
                          login_ip          varchar(50)     default ''                 comment '最后登陆IP',
                          login_date        datetime                                   comment '最后登陆时间',
                          create_by         varchar(64)     default ''                 comment '创建者',
                          create_time 	    datetime NOT NULL ON UPDATE CURRENT_TIMESTAMP    comment '创建时间',
                          update_by         varchar(64)     default ''                 comment '更新者',
                          update_time       datetime                                   comment '更新时间',
                          remark 		    text			   comment '备注',
                          primary key (user_id)
) engine=innodb comment = '用户信息表';



-- ----------------------------
-- 3、岗位信息表
-- ----------------------------
drop table if exists sys_post;
create table sys_post
(
    post_id       varchar(32)         not null     comment '岗位ID',
    post_code     varchar(64)     not null                   comment '岗位编码',
    post_name     varchar(50)     not null                   comment '岗位名称',
    post_sort     int(4)          not null                   comment '显示顺序',
    status        char(1)         not null                   comment '状态（0正常 1停用）',
    create_by     varchar(64)     default ''                 comment '创建者',
    create_time   datetime     NOT NULL ON UPDATE CURRENT_TIMESTAMP     comment '创建时间',
    update_by     varchar(64) 	  default ''			     comment '更新者',
    update_time   datetime                                   comment '更新时间',
    remark 		  text 	  				 comment '备注',
    primary key (post_id)
) engine=innodb comment = '岗位信息表';



-- ----------------------------
-- 4、角色信息表
-- ----------------------------
drop table if exists sys_role;
create table sys_role (
                          role_id 			varchar(32)		not null     comment '角色ID',
                          role_name 		varchar(30) 	not null 				   comment '角色名称',
                          role_key 		    varchar(100) 	not null 				   comment '角色权限字符串',
                          role_sort         int(4)          not null                   comment '显示顺序',
                          data_scope        char(1) 	    default '1'				   comment '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）',
                          status 			char(1) 		not null 			       comment '角色状态（0正常 1停用）',
                          del_flag			char(1) 		default '0' 			   comment '删除标志（0代表存在 2代表删除）',
                          create_by         varchar(64)     default ''                 comment '创建者',
                          create_time 		datetime     NOT NULL ON UPDATE CURRENT_TIMESTAMP  comment '创建时间',
                          update_by 		varchar(64) 	default ''			       comment '更新者',
                          update_time 		datetime                                   comment '更新时间',
                          remark 			text  				   comment '备注',
                          primary key (role_id)
) engine=innodb  comment = '角色信息表';



-- ----------------------------
-- 5、菜单权限表
-- ----------------------------
drop table if exists sys_menu;
create table sys_menu (
                          menu_id 			varchar(32) 		not null     comment '菜单ID',
                          menu_name 		varchar(50) 	not null 				   comment '菜单名称',
                          parent_id 		varchar(32) 		default 0 			       comment '父菜单ID',
                          order_num 		int(4) 			default 0 			       comment '显示顺序',
                          url 				varchar(200) 	default '#'				   comment '请求地址',
                          target            varchar(20)     default ''                 comment '打开方式（menuItem页签 menuBlank新窗口）',
                          menu_type 		char(1) 		default '' 			       comment '菜单类型（M目录 C菜单 F按钮）',
                          visible 			char(1) 		default 0 				   comment '菜单状态（0显示 1隐藏）',
                          perms 			varchar(100) 	default '' 				   comment '权限标识',
                          icon 				varchar(100) 	default '#' 			   comment '菜单图标',
                          create_by         varchar(64)     default ''                 comment '创建者',
                          create_time 		datetime NOT NULL ON UPDATE CURRENT_TIMESTAMP  comment '创建时间',
                          update_by 		varchar(64) 	default ''			       comment '更新者',
                          update_time 		datetime                                   comment '更新时间',
                          remark 			text 				   comment '备注',
                          primary key (menu_id)
) engine=innodb comment = '菜单权限表';


-- ----------------------------
-- 6、用户和角色关联表  用户N-1角色
-- ----------------------------
drop table if exists sys_user_role;
create table sys_user_role (
                               user_id 	varchar(32) not null comment '用户ID',
                               role_id 	varchar(32) not null comment '角色ID',
                               primary key(user_id, role_id)
) engine=innodb comment = '用户和角色关联表';



-- ----------------------------
-- 7、角色和菜单关联表  角色1-N菜单
-- ----------------------------
drop table if exists sys_role_menu;
create table sys_role_menu (
                               role_id 	varchar(32) not null comment '角色ID',
                               menu_id 	varchar(32) not null comment '菜单ID',
                               primary key(role_id, menu_id)
) engine=innodb comment = '角色和菜单关联表';


-- ----------------------------
-- 8、角色和部门关联表  角色1-N部门
-- ----------------------------
drop table if exists sys_role_dept;
create table sys_role_dept (
                               role_id 	varchar(32) not null comment '角色ID',
                               dept_id 	varchar(32) not null comment '部门ID',
                               primary key(role_id, dept_id)
) engine=innodb comment = '角色和部门关联表';


-- ----------------------------
-- 9、用户与岗位关联表  用户1-N岗位
-- ----------------------------
drop table if exists sys_user_post;
create table sys_user_post
(
    user_id varchar(32) not null comment '用户ID',
    post_id varchar(32) not null comment '岗位ID',
    primary key (user_id, post_id)
) engine=innodb comment = '用户与岗位关联表';




-- ----------------------------
-- 10、操作日志记录
-- ----------------------------
drop table if exists sys_oper_log;
create table sys_oper_log (
                              oper_id 			varchar(32) 		not null     comment '日志主键',
                              title             varchar(50)     default ''                 comment '模块标题',
                              business_type     int(2)          default 0                  comment '业务类型（0其它 1新增 2修改 3删除）',
                              method            varchar(100)    default ''                 comment '方法名称',
                              operator_type     int(1)          default 0                  comment '操作类别（0其它 1后台用户 2手机端用户）',
                              oper_name 	    varchar(50)     default '' 		 	 	   comment '操作人员',
                              dept_name 		varchar(50)     default '' 		 	 	   comment '部门名称',
                              oper_url 		    varchar(255) 	default '' 				   comment '请求URL',
                              oper_ip 			varchar(50) 	default '' 				   comment '主机地址',
                              oper_location     varchar(255)    default ''                 comment '操作地点',
                              oper_param 		text 				   comment '请求参数',
                              status 			int(1) 		    default 0				   comment '操作状态（0正常 1异常）',
                              error_msg 		text 				   comment '错误消息',
                              oper_time 		datetime                                   comment '操作时间',
                              primary key (oper_id)
) engine=innodb  comment = '操作日志记录';


-- ----------------------------
-- 11、字典类型表
-- ----------------------------
drop table if exists sys_dict_type;
create table sys_dict_type
(
    dict_id          varchar(32) 		 not null     comment '字典主键',
    dict_name        varchar(100)    default ''                 comment '字典名称',
    dict_type        varchar(100)    default ''                 comment '字典类型',
    status 			 char(1) 		 default '0'			    comment '状态（0正常 1停用）',
    create_by        varchar(64)     default ''                 comment '创建者',
    create_time      datetime      NOT NULL ON UPDATE CURRENT_TIMESTAMP   comment '创建时间',
    update_by        varchar(64) 	 default ''			        comment '更新者',
    update_time      datetime                                   comment '更新时间',
    remark 	         text 				comment '备注',
    primary key (dict_id),
    unique (dict_type)
) engine=innodb  comment = '字典类型表';



-- ----------------------------
-- 12、字典数据表
-- ----------------------------
drop table if exists sys_dict_data;
create table sys_dict_data
(
    dict_code        varchar(32) 		 not null    comment '字典编码',
    dict_sort        int(4)          default 0                  comment '字典排序',
    dict_label       varchar(100)    default ''                 comment '字典标签',
    dict_value       varchar(100)    default ''                 comment '字典键值',
    dict_type        varchar(100)    default ''                 comment '字典类型',
    css_class        varchar(100)    default ''                 comment '样式属性（其他样式扩展）',
    list_class       varchar(100)    default ''                 comment '表格回显样式',
    is_default       char(1)         default 'N'                comment '是否默认（Y是 N否）',
    status 			     char(1) 		 default '0'			    comment '状态（0正常 1停用）',
    create_by        varchar(64)     default ''                 comment '创建者',
    create_time      datetime   NOT NULL ON UPDATE CURRENT_TIMESTAMP comment '创建时间',
    update_by        varchar(64) 	 default ''			        comment '更新者',
    update_time      datetime                                   comment '更新时间',
    remark 	         text 				comment '备注',
    primary key (dict_code)
) engine=innodb  comment = '字典数据表';



-- ----------------------------
-- 13、参数配置表
-- ----------------------------
drop table if exists sys_config;
create table sys_config (
                            config_id 		     varchar(32) 	     not null     comment '参数主键',
                            config_name        varchar(100)  default ''                 comment '参数名称',
                            config_key         varchar(100)  default ''                 comment '参数键名',
                            config_value       varchar(100)  default ''                 comment '参数键值',
                            config_type        char(1)       default 'N'                comment '系统内置（Y是 N否）',
                            create_by          varchar(64)   default ''                 comment '创建者',
                            create_time 	     datetime   NOT NULL ON UPDATE CURRENT_TIMESTAMP    comment '创建时间',
                            update_by          varchar(64)   default ''                 comment '更新者',
                            update_time        datetime                                 comment '更新时间',
                            remark 	           text 				comment '备注',
                            primary key (config_id)
) engine=innodb  comment = '参数配置表';



-- ----------------------------
-- 14、系统访问记录
-- ----------------------------
drop table if exists sys_logininfor;
create table sys_logininfor (
                                info_id 		   varchar(32) 	   not null    comment '访问ID',
                                login_name 	   varchar(50)   default '' 			     comment '登录账号',
                                ipaddr 		     varchar(50)   default '' 			     comment '登录IP地址',
                                login_location varchar(255)  default ''                comment '登录地点',
                                browser  		 varchar(50)   default '' 			     comment '浏览器类型',
                                os      		 varchar(50)   default '' 			     comment '操作系统',
                                status 		 char(1) 	   default '0' 			     comment '登录状态（0成功 1失败）',
                                msg      		 text 			     comment '提示消息',
                                login_time 	 datetime                                comment '访问时间',
                                primary key (info_id)
) engine=innodb  comment = '系统访问记录';


-- ----------------------------
-- 15、在线用户记录
-- ----------------------------
drop table if exists sys_user_online;
create table sys_user_online (
                                 sessionId 	    varchar(50)  default ''              	comment '用户会话id',
                                 login_name 	    varchar(50)  default '' 		 	 	comment '登录账号',
                                 dept_name 		varchar(50)  default '' 		 	 	comment '部门名称',
                                 ipaddr 		    varchar(50)  default '' 			 	comment '登录IP地址',
                                 login_location    varchar(255) default ''                 comment '登录地点',
                                 browser  		    varchar(50)  default '' 			 	comment '浏览器类型',
                                 os      		    varchar(50)  default '' 			 	comment '操作系统',
                                 status      	    varchar(10)  default '' 			 	comment '在线状态on_line在线off_line离线',
                                 start_timestamp 	datetime                                comment 'session创建时间',
                                 last_access_time  datetime                                comment 'session最后访问时间',
                                 expire_time 	    int(5) 		 default 0 			 	    comment '超时时间，单位为分钟',
                                 primary key (sessionId)
) engine=innodb comment = '在线用户记录';


-- ----------------------------
-- 16、定时任务调度表
-- ----------------------------
drop table if exists sys_job;
create table sys_job (
                         job_id 		      varchar(32) 	    not null     comment '任务ID',
                         job_name            varchar(64)   default ''                 comment '任务名称',
                         job_group           varchar(64)   default ''                 comment '任务组名',
                         method_name         varchar(500)  default ''                 comment '任务方法',
                         method_params       text  default null                 comment '方法参数',
                         cron_expression     varchar(255)  default ''                 comment 'cron执行表达式',
                         misfire_policy      varchar(20)   default '3'                comment '计划执行错误策略（1立即执行 2执行一次 3放弃执行）',
                         concurrent          char          default '1'                comment '是否并发执行（0允许 1禁止）',
                         status              char(1)       default '0'                comment '状态（0正常 1暂停）',
                         create_by           varchar(64)   default ''                 comment '创建者',
                         create_time         datetime    NOT NULL ON UPDATE CURRENT_TIMESTAMP  comment '创建时间',
                         update_by           varchar(64)   default ''                 comment '更新者',
                         update_time         datetime                                 comment '更新时间',
                         remark              text                 comment '备注信息',
                         primary key (job_id, job_name, job_group)
) engine=innodb  comment = '定时任务调度表';



-- ----------------------------
-- 17、定时任务调度日志表
-- ----------------------------
drop table if exists sys_job_log;
create table sys_job_log (
                             job_log_id          varchar(32) 	    not null     comment '任务日志ID',
                             job_name            varchar(64)   not null                   comment '任务名称',
                             job_group           varchar(64)   not null                   comment '任务组名',
                             method_name         varchar(500)                             comment '任务方法',
                             method_params       text  default null                 comment '方法参数',
                             job_message         text                             comment '日志信息',
                             status              char(1)       default '0'                comment '执行状态（0正常 1失败）',
                             exception_info      text                 comment '异常信息',
                             create_time         datetime  NOT NULL ON UPDATE CURRENT_TIMESTAMP    comment '创建时间',
                             primary key (job_log_id)
) engine=innodb comment = '定时任务调度日志表';


-- ----------------------------
-- 18、通知公告表
-- ----------------------------
drop table if exists sys_notice;
create table sys_notice (
                            notice_id 		varchar(32) 		    not null     comment '公告ID',
                            notice_title 		varchar(50) 	not null 				   comment '公告标题',
                            notice_type 		char(1) 	    not null 			       comment '公告类型（1通知 2公告）',
                            notice_content    text    not null                   comment '公告内容',
                            status 			char(1) 		default '0' 			   comment '公告状态（0正常 1关闭）',
                            create_by         varchar(64)     default ''                 comment '创建者',
                            create_time 		datetime     NOT NULL ON UPDATE CURRENT_TIMESTAMP  comment '创建时间',
                            update_by 		varchar(64) 	default ''			       comment '更新者',
                            update_time 		datetime                                   comment '更新时间',
                            remark 			text 				   comment '备注',
                            primary key (notice_id)
) engine=innodb  comment = '通知公告表';

