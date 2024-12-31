mod google_client;
mod route;
mod structs;

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use dotenv::dotenv;

use google_client::GoogleClient;
use http::{header::CONTENT_TYPE, Method, StatusCode};
use oauth_axum::{providers::google::GoogleProvider, CustomProvider, OAuthClient};
use serde_json::Value;

use route::create_router;
use structs::{AppState, QueryAxumCallback};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_origin(Any)
        .allow_headers(vec![CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState::new())).layer(cors);

    println!("Server started successfully at 127.0.0.1:3000");

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn get_client() -> CustomProvider {
    GoogleProvider::new(
        std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
        std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
        std::env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set"),
    )
}

pub async fn create_url(State(state): State<Arc<AppState>>) -> String {
    let state_oauth = get_client()
        .generate_url(
            Vec::from([
                std::env::var("GOOGLE_EMAIL_SCOPE").expect("GOOGLE_EMAIL_SCOPE must be set"),
                std::env::var("GOOGLE_PROFILE_SCOPE").expect("GOOGLE_PROFILE_SCOPE must be set"),
            ]),
            |state_e| async move {
                // SAVE THE DATA IN THE DB OR MEMORY
                // state should be your ID
                state.oauth_state.set(state_e.state, state_e.verifier).await;
            },
        )
        .await
        .ok()
        .unwrap()
        .state
        .unwrap();

    state_oauth.url_generated.unwrap()
}

pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(queries): Query<QueryAxumCallback>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let oauth_verifier = state
        .oauth_state
        .get(queries.state.clone())
        .await
        .ok_or((StatusCode::BAD_REQUEST, "Invalid state".to_string()))?;

    let access_token = get_client()
        .generate_token(queries.code, oauth_verifier)
        .await
        .ok()
        .unwrap();

    println!("{:?}", access_token);

    let google_client = GoogleClient::new(&state.http_client);
    let user_info_result = google_client.fetch_user_info(&access_token).await;

    match user_info_result {
        Ok(user_info) => Ok(Json(user_info)),
        Err((status, error_message)) => Err((status, error_message)),
    }
}
