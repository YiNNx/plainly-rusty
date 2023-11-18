mod client;
mod error;
mod github;

#[cfg(test)]
mod tests {
    use crate::client::OauthClient;
    use crate::github::GithubClient;
    use std::env;

    #[test]
    fn test() {
        let client_id = env::var("CLIENT_ID").unwrap();
        let client_secret = env::var("CLIENT_SECRET").unwrap();
        let code = env::var("CODE").unwrap();

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            println!("== test started ==");
            let client = GithubClient::new(
                client_id,
                client_secret,
                "plainly-rusty".to_string(),
            );
            let token = client
                .exchange_code(code)
                .await
                .unwrap();
            let res = client.access_user_resource(token).await.unwrap();
            println!("uid: {:?}", res);
            println!("== test finished ==");
        });
    }
}
