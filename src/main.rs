use std::error::Error;

use axum::http::StatusCode;
use axum::Router;
use axum::routing::post;

pub const API_V1: &str = "/api/v1";

#[utoipa::path(
post,
path = format!("{}/login", API_V1),
responses(
(status = 200, description = "Return user with permissions", body = User)
)
)]
pub async fn post_login() -> Result<(), (StatusCode, String)> {
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route(format!("{}/login", API_V1).as_str(), post(post_login));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
