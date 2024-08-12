use actix_web::web::{get, scope, ServiceConfig};

mod login;

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/auth").route("login", get().to(login::login)));
}
