use crate::{users::User, AppState};

use axum::{
    extract::ConnectInfo,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::net::SocketAddr;

pub async fn token_bucket<B>(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut users = state.lock().await;
    let user_ip = addr;

    if let Some(user) = users.iter().find(|user| user.addr == user_ip) {
        println!("Existing user");
        if user.bucket_is_empty().await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        } else {
            user.consume().await;
        }
    } else {
        println!("New user");
        let user: User = User::new(user_ip).await;
        users.push(user);
    }

    let resp = next.run(req).await;
    Ok(resp)
}
