use reqwest::{Client, StatusCode};
use std::error::Error;

lazy_static::lazy_static! { static ref REQ_CLIENT:Client = Client::builder().build().unwrap(); }

#[derive(serde::Serialize)]
struct ReqExchangeCode {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RespExchangeCode {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[async_trait::async_trait]
pub trait OauthClient<T: serde::de::DeserializeOwned> {
    fn new(client_id: String, client_secret: String, user_agent: String) -> Self;

    fn client_id(&self) -> &String;
    fn client_secret(&self) -> &String;
    fn user_agent(&self) -> &String;
    fn url_token(&self) -> &String;
    fn url_resource(&self) -> &String;

    async fn exchange_code(&self, code: String) -> Result<String, Box<dyn Error>> {
        let request_body = ReqExchangeCode {
            client_id: self.client_id().clone(),
            client_secret: self.client_secret().clone(),
            code: code,
        };
        let resp = REQ_CLIENT
            .post(self.url_token())
            .header("Accept", "application/json")
            .header("User-Agent", self.user_agent())
            .json(&request_body)
            .send()
            .await?;

        let resp_token: RespExchangeCode = resp.json().await?;
        if let Some(token) = resp_token.access_token {
            Ok(token)
        } else {
            if let Some(err_msg) = resp_token.error {
                Err(Box::new(crate::error::ErrorCustom(err_msg)))
            } else {
                Err(Box::new(crate::error::ErrorCustom(
                    "unexpected error to get response token".to_string(),
                )))
            }
        }
    }

    async fn access_user_resource(&self, access_token: String) -> Result<T, Box<dyn Error>> {
        let resp = REQ_CLIENT
            .get(self.url_resource())
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", self.user_agent())
            .send()
            .await?;

        if resp.status() != StatusCode::OK {
            let resp_text = resp.text().await?;
            return Err(Box::new(crate::error::ErrorCustom(resp_text)));
        }
        let resp_resource: T = resp.json().await?;
        Ok(resp_resource)
    }
}
