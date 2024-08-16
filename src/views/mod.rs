use actix_web::web::ServiceConfig;
use auth::auth_views_factory;

pub mod auth;
pub mod to_do;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
}
