#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub application: Application,
    pub graphql: GraphQL,
    pub database: Database,
    pub oauth_github: OauthGithub,
    pub rss_channel: RssChannel,
}

#[derive(serde::Deserialize, Debug)]
pub struct Application {
    pub address: String,
    pub debug: bool,
    pub owner_github_name: String,
    pub url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GraphQL {
    pub endpoint: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Database {
    pub url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct OauthGithub {
    pub client_id: String,
    pub client_secret: String,
    pub user_agent: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RssChannel {
    pub title: String,
    pub description: String,
    pub copyright: String,
}
