use crate::error::{Error, Result};
use crate::routes::AUTH_TOKEN;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - login", "HANDLER");

    // TODO: implement a real db login

    if payload.username != "demo" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }
    // todo!();

    // TODO: set cookies
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    // create success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

pub fn auth() -> Router {
    Router::new().route("/login", post(login))
}
