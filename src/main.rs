use axum::{
    middleware::{self as axumMiddleware},
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // create /limited route with the middleware
    let limited_token_bucket = Router::new()
        .route("/limited", get(service::limited))
        .layer(axumMiddleware::from_fn(middleware::token_bucket));

    // build our application with a route
    let app = Router::new()
        .route("/", get(service::root))
        .route("/unlimited", get(service::unlimited))
        .merge(limited_token_bucket);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

mod middleware;
mod service;
