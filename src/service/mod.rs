pub mod user_service;
pub mod redis_service;

use crate::config::REDIS_URL;
use crate::service::redis_service::RedisService;

lazy_static! {
   /// redis
   pub static ref REDIS_SERVICE: RedisService = RedisService::new(&REDIS_URL);
}