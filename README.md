# axum-google-oauth

`axum-google-oauth` is a barebones example implementation of the `oauth-axum` crate for authenticating with the [Google OAuth 2.0 API](https://developers.google.com/identity/protocols/oauth2) and retrieving user information.

More information about this crate can be found in the [crate documentation](https://crates.io/crates/oauth-axum).

### Implementation

1. Create an OAuth 2.0 Client ID and Client Secret in the Google Cloud Platform Console by following these instructions. See [here](https://support.google.com/cloud/answer/6158849?hl=en).
2. Clone the repository.
3. Copy the `.env.example` file, rename it to `.env`, and copy across your Client ID and Client Secret obtained in step 1.
4. Modify the implementation to suit your requirements.

### How to use

1. Perform `cargo run` in the repository.
   - Use [cargo watch](https://crates.io/crates/cargo-watch) to enable live reloading.
2. Make a GET request to the `create_url` endpoint:
   - `curl http://localhost:3000/`
3. Open the returned URL in your browser.
4. Login to your google account.
5. See the user information in the callback.
