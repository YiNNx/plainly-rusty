use std::error::Error;

#[async_trait::async_trait]
pub trait OauthClient {
    fn new(client_id: String, client_secret: String) -> Self;
    async fn exchange_code(&mut self, code: String) -> Result<String, Box<dyn Error>>;
    async fn access_user_info(&self,access_token:String) -> Result<isize, Box<dyn Error>>;
}
