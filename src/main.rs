use axum::{
    Router,
    extract::Path,
    http::{Response, StatusCode},
    routing::get,
};

async fn greet_base() -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .body("Hello, World!".into())
        .unwrap()
}

async fn greet(Path(name): Path<String>) -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .body(format!("Hello, {}!", name))
        .unwrap()
}

#[tokio::main]
async fn main() {
    // build our app with a couple routes
    let app = Router::new()
        .route("/", get(greet_base))
        .route("/{name}", get(greet));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
