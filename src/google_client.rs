use http::StatusCode;
use reqwest::Client;
use serde_json::Value;

pub struct GoogleClient<'a> {
    http_client: &'a Client,
}

impl<'a> GoogleClient<'a> {
    pub fn new(http_client: &'a Client) -> Self {
        GoogleClient { http_client }
    }

    pub async fn fetch_user_info(&self, access_token: &str) -> Result<Value, (StatusCode, String)> {
        let response = self
            .http_client
            .get("https://people.googleapis.com/v1/people/me?personFields=names,emailAddresses")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Request failed: {}", err),
                )
            })?;

        if !response.status().is_success() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Failed to fetch user info".to_string(),
            ));
        }

        response.json::<Value>().await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to parse response: {}", err),
            )
        })
    }
}
