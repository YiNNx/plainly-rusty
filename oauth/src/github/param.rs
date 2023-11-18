#[derive(serde::Serialize)]
pub struct ReqToken {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespToken {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespUserInfo {
    pub id: Option<isize>,
    pub message: Option<String>,
}
