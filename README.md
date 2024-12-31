# axum-google-oauth

`axum-google-oauth` is a barebones example implementation of the `oauth-axum` crate for authenticating with the [Google OAuth 2.0 API](https://developers.google.com/identity/protocols/oauth2) and retrieving user information.

More information about this create can be found in the [crate documentation](https://crates.io/crates/oauth-axum).

### Implementation

1. Create an OAuth 2.0 Client ID and Client Secret in the Google Cloud Platform Console by following these instructions. See [here](https://support.google.com/cloud/answer/6158849?hl=en).
2. Clone the repository.
3. Copy the `.env.example` file, rename it to `.env`, and copy across your Client ID and Client Secret obtained in step 1.
4. Modify the implementation to suite your requirements.
