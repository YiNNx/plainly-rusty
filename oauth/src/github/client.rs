use reqwest::Client;

use super::param::{TokenRequest, TokenResponse, UserInfoResponse};

pub async fn github() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "";
    let client_secret = "";

    let client = Client::builder().build()?;

    let token_url = "https://github.com/login/oauth/access_token";
    let request_body = TokenRequest {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        code: "".to_string(),
    };
    let response = client
        .post(token_url)
        .header("Accept", "application/json")
        .json(&request_body)
        .send()
        .await?;
    let token_response: TokenResponse = response.json().await?;
    print!("{:?}", token_response);

    let access_token = token_response.access_token.unwrap();

    let user_url = "https://api.github.com/user";
    let response = client
        .get(user_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("User-Agent", "just-plain.fun/0.1.0")
        .send()
        .await?;

    let user: UserInfoResponse = response.json().await?;
    println!("{:?}", user);
    Ok(())
}
