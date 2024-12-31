use std::collections::HashMap;

use reqwest::Client;

use tokio::sync::Mutex;

pub struct OAuthState {
    pub data: Mutex<HashMap<String, String>>,
}

impl OAuthState {
    fn new() -> Self {
        OAuthState {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub async fn set(&self, state: String, verifier: String) {
        let mut data = self.data.lock().await;
        data.insert(state, verifier);
    }

    pub async fn get(&self, state: String) -> Option<String> {
        let data = self.data.lock().await;
        data.get(&state).cloned()
    }

    pub async fn get_all_items(&self) -> HashMap<String, String> {
        let data = self.data.lock().await;
        data.clone()
    }
}

pub struct AppState {
    pub http_client: Client,
    pub oauth_state: OAuthState,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            http_client: Client::new(),
            oauth_state: OAuthState::new(),
        }
    }
}

#[derive(Clone, serde::Deserialize)]
pub struct QueryAxumCallback {
    pub code: String,
    pub state: String,
}
