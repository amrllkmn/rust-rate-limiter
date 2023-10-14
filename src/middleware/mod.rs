use crate::{users::User, AppState};
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

pub async fn token_bucket<B>(
    State(state): State<AppState>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("The middleware came first");
    let header = req.headers();
    // I know this is dirty, but we can fix it later
    let user_id = header
        .get("userid")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut users = state.lock().await;

    println!("{:?}", users);

    if let Some(user) = users.iter_mut().find(|user| user.id == user_id) {
        if user.bucket_is_empty() {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
        user.consume();

        println!("The user: {:?}", user);
    } else {
        let mut user: User = User::new(user_id);

        user.start();

        users.push(user);
    }

    let resp = next.run(req).await;
    Ok(resp)
}
