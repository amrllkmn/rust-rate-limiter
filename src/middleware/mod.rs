use axum::{http::Request, middleware::Next, response::Response};

pub async fn token_bucket<B>(req: Request<B>, next: Next<B>) -> Response {
    println!("The middleware came first");
    next.run(req).await
}
