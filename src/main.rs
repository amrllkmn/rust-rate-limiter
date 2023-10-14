use axum::{
    extract::State,
    middleware::{self as axumMiddleware},
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};

use tokio::sync::Mutex;

type AppState = Arc<Mutex<Vec<users::User>>>;

/// We need a persistent state that:
/// - Keeps track of the users
/// - Adds new users to the state
/// - Handles refreshing token at a specified rate
/// Solution:
/// - Offload the tracking to a struct

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // create state
    let state: AppState = Arc::new(Mutex::new(Vec::new()));

    // create /limited and /unlimited behind /api route with the middleware
    let api = Router::new()
        .route("/limited", get(service::limited))
        .layer(axumMiddleware::from_fn_with_state(
            state.clone(),
            middleware::token_bucket,
        ))
        .route("/unlimited", get(service::unlimited));

    // build our application with a route
    let app = Router::new()
        .route("/", get(service::root))
        .nest("/api", api)
        .with_state(state);

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
mod users;
