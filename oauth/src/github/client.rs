use reqwest::Client;

use super::param::{ReqToken, RespToken, RespUserInfo};
use crate::client::OauthClient;

lazy_static::lazy_static! { static ref CLIENT:Client = Client::builder().build().unwrap(); }

const GITHUB_URL_TOKEN: &str = "https://github.com/login/oauth/access_token";
const GITHUB_URL_USER: &str = "https://github.com/login/oauth/access_token";

pub struct GithubClient {
    client_id: String,
    client_secret: String,
    url_token: String,
    url_user: String,
}

#[async_trait::async_trait]
impl OauthClient for GithubClient {
    fn new(client_id: String, client_secret: String) -> Self {
        return GithubClient {
            client_id: client_id,
            client_secret: client_secret,
            url_token: GITHUB_URL_TOKEN.to_string(),
            url_user: GITHUB_URL_USER.to_string(),
        };
    }

    async fn exchange_code(&mut self, code: String) -> Result<String, Box<dyn std::error::Error>> {
        let request_body = ReqToken {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            code: code,
        };
        let resp: RespToken = CLIENT
            .post(&self.url_token)
            .header("Accept", "application/json")
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        if let Some(token) = resp.access_token {
            Ok(token)
        } else {
            if let Some(err_msg) = resp.error {
                Err(Box::new(crate::error::Error(err_msg)))
            } else {
                Err(Box::new(crate::error::Error(
                    "unexpected error to get response token".to_string(),
                )))
            }
        }
    }

    async fn access_user_info(
        &self,
        access_token: String,
    ) -> Result<isize, Box<dyn std::error::Error>> {
        let resp: RespUserInfo = CLIENT
            .get(&self.url_user)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "just-plain.fun/0.1.0")
            .send()
            .await?
            .json()
            .await?;

        if let Some(token) = resp.id {
            Ok(token)
        } else {
            if let Some(err_msg) = resp.message {
                Err(Box::new(crate::error::Error(err_msg)))
            } else {
                Err(Box::new(crate::error::Error(
                    "unexpected error to get response token".to_string(),
                )))
            }
        }
    }
}
