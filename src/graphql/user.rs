use async_graphql::{dynamic::*, Value};
use seaography::{CustomMutation, CustomMutationArgument};

use crate::config::global_config;
use crate::utilities::jwt::Role::{Owner, User};
use oauth::{client::OauthClient, github::GithubClient};

lazy_static::lazy_static! {
    static ref OAUTH_CLIENT : GithubClient =GithubClient::new(
        &global_config().oauth_github.client_id,
        &global_config().oauth_github.client_secret,
        &global_config().oauth_github.user_agent,
    );
}

pub fn mutation_grant_token() -> CustomMutation {
    CustomMutation {
        name: "tokenGrant".into(),
        arguments: vec![CustomMutationArgument {
            name: "code".into(),
            ty: TypeRef::named_nn(TypeRef::STRING),
        }],
        ty: TypeRef::named_nn(TypeRef::STRING),
        resolver_fn: Box::new(|ctx| {
            FieldFuture::new(async move {
                let code = ctx.args.try_get("code")?.string()?.to_string();
                let res = OAUTH_CLIENT.code2resource(code).await;
                let user_info = match res {
                    Ok(u) => u,
                    Err(e) => return Err(async_graphql::Error::new(e.to_string())),
                };
                let id = match user_info.id {
                    Some(id) => id,
                    None => return Err(async_graphql::Error::new("uncaught error".to_string())),
                };
                let role = if id == global_config().application.owner_github_id {
                    Owner
                } else {
                    User
                };
                let jwt = crate::utilities::jwt::create_jwt(id, role)?;
                Ok(Some(Value::from(jwt)))
            })
        }),
    }
}
