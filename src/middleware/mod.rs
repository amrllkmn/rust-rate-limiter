use crate::{users::User, AppState};
use axum::{extract::State, http::Request, middleware::Next, response::Response};

pub async fn token_bucket<B>(
    State(state): State<AppState>,
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let header = req.headers();
    // I know this is dirty, but we can
    let userId = header
        .get("userid")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let user: User = User::new(userId);
    println!("The middleware came first");
    next.run(req).await
}
