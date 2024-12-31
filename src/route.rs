use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{callback, create_url, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(create_url))
        .route("/callback", get(callback))
        .with_state(app_state)
}
