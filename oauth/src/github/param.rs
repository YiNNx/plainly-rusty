#[derive(serde::Serialize)]
pub struct TokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct UserInfoResponse {
    pub id: Option<isize>,
    pub message: Option<String>,
}
