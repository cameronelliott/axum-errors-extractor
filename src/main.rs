//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-hello-world
//! ```

use axum::extract::Path;
use axum::http::{Request, StatusCode};
use axum::{
    body::Body,
    error_handling::HandleError,
    response::{Html, Response},
    routing::get,
    Router,
};
use std::net::SocketAddr;

async fn thing_that_might_fail(
    Path(key): Path<String>,
    r: Request<Body>,
) -> Result<String, anyhow::Error> {
    Ok("hello".to_string())
}

// handle errors by converting them into something that implements
// `IntoResponse`
async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}

#[tokio::main]
async fn main() {
    // this service might fail with `anyhow::Error`
    let some_fallible_service = tower::service_fn(|req| async {
        thing_that_might_fail(req).await?;
        Ok::<_, anyhow::Error>(Response::new(Body::empty()))
    });

    let app = Router::new().route(
        "/",
        HandleError::new(some_fallible_service, handle_anyhow_error),
    );

    // build our application with a route
    //   let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
