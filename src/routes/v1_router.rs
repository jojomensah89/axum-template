use crate::controllers::user_controller::{create_user, get_users, handler_hello, look, root};
use crate::routes::login_route;
use axum::{extract::Path, routing::get, Router};
// use std::error::Error;
// use serde::Serialize;
// use std::sync::Arc;
// use tokio::sync::RwLock;
use tower_cookies::CookieManagerLayer;

pub fn v1_router() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/hello", get(handler_hello))
        .route("/look", get(look))
        // `POST /users` goes to `create_user` and `GET /users` goes to `get_users`
        .route("/users", get(get_users).post(create_user))
        .merge(launches_router())
        .route("/planet/:name", get(planet_controller))
        .nest("/v1", planets_router())
        .merge(login_route::auth())
        // .merge(route_tickets::ticket_route(mc))
        .layer(CookieManagerLayer::new())
}

pub fn launches_router() -> Router {
    async fn launches_controller() -> &'static str {
        "launches route"
    }
    Router::new().route("/launches", get(launches_controller))
}
pub fn planets_router() -> Router {
    async fn planets_controller() -> &'static str {
        "planets route"
    }
    Router::new().route("/planets", get(planets_controller))
}

async fn planet_controller(Path(name): Path<String>) -> String {
    format!("the name of the planet is: {}", name)
}
