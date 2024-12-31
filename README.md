# axum-google-oauth

`axum-google-oauth` is an barebones example implementation of the `oauth-axum` crate for authenticating with the Google OAuth 2.0 API and retrieving user information.

More information about this create can be found in the [crate documentation](https://crates.io/crates/oauth-axum).

Note: This repository is intended to be used as an example for implementing your own Google OAuth in a Rust application.

### Implementation

1. Create an OAuth 2.0 Client ID in the Google Cloud Platform Console by following these instructions. See [here](https://support.google.com/cloud/answer/6158849?hl=en).
2. Clone the repository.
3. Copy the `.env.example` file, rename it to `.env`, and copy across your Client ID and client Secret obtained in step 1.
4. Modify the implementation to suite your requirements.
