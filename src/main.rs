#![allow(unused)]

mod ctx;
mod error;
mod log;
mod model;
mod util;
mod web;

use crate::{log::log_request, model::ModelController};
use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    serve, Json, Router,
};
use ctx::Ctx;
pub use error::{Error, Result};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let model_controller = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(model_controller.clone()).route_layer(
        middleware::from_fn(web::middleware_auth::middleware_require_auth),
    );

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            model_controller.clone(),
            web::middleware_auth::middleware_context_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:		--- Start Server

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!(
        "Listening at address: http://{:?}",
        listener.local_addr().unwrap()
    );

    serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    // endregion:	--- Start Server

    Ok(())
}

async fn main_response_mapper(
    context: Option<Ctx>,
    uri: Uri,
    request_method: Method,
    response: Response,
) -> Response {
    println!("-->> {:<20} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    // Get the eventual response error
    let service_error = response.extensions().get::<Error>();

    let client_stats_error = service_error.map(|s| s.client_status_and_error());

    // If client error, build the new response.
    let error_response = client_stats_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("-->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_stats_error.unzip().1;

    log_request(
        uuid,
        request_method,
        uri,
        context,
        service_error,
        client_error,
    );

    println!("-->> server log line - {uuid} - Error: {service_error:?}");

    println!();

    error_response.unwrap_or(response)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:		--- Routes Hello

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("-->> {:<20} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("-->> {:<20} - handler_hello", "HANDLER");

    Html(format!("Hello <strong>{name}!!!</strong>"))
}

// endregion:	--- Routes Hello
