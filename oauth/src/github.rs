use crate::client::OauthClient;

const GITHUB_URL_TOKEN: &str = "https://github.com/login/oauth/access_token";
const GITHUB_URL_USER: &str = "https://api.github.com/user";

pub struct GithubClient {
    client_id: String,
    client_secret: String,
    user_agent: String,
    url_token: String,
    url_resource: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GithubUserInfo {
    pub id: Option<usize>,
    pub login: Option<String>,
    pub avatar_url: Option<String>,
    pub name: Option<String>,
    pub message: Option<String>,
}

#[async_trait::async_trait]
impl OauthClient<GithubUserInfo> for GithubClient {
    fn new(client_id: String, client_secret: String, user_agent: String) -> Self {
        Self {
            client_id,
            client_secret,
            user_agent,
            url_token: GITHUB_URL_TOKEN.to_string(),
            url_resource: GITHUB_URL_USER.to_string(),
        }
    }

    fn client_id(&self) -> &String {
        &self.client_id
    }

    fn client_secret(&self) -> &String {
        &self.client_secret
    }

    fn user_agent(&self) -> &String {
        &self.user_agent
    }

    fn url_token(&self) -> &String {
        &self.url_token
    }

    fn url_resource(&self) -> &String {
        &self.url_resource
    }
}
