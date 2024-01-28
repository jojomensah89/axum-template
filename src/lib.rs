mod controllers;
mod ctx;
mod error;
mod log;
mod middlewares;
mod model;
mod routes;

use axum::http::{Method, Uri};
use uuid::Uuid;

use axum::{middleware, Json};
use serde_json::json;

use crate::ctx::Ctx;
use axum::response::{IntoResponse, Response};

use tower_cookies::CookieManagerLayer;

pub use self::error::{Error, Result};
use crate::log::log_request;
use crate::model::model::ModelController;
use crate::routes::route_tickets;
use crate::routes::v1_router::v1_router;
use middlewares::mw_auth;
pub async fn run() -> Result<()> {
    let mc = ModelController::new().await?;

    //Add middleware to specific route
    let ticket_routes = route_tickets::ticket_route(mc.clone())
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    let app = v1_router()
        .nest("/api", ticket_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new());
    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!(" -->> Listening on port {:?}\n", listener.local_addr());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}
