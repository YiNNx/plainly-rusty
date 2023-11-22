use crate::entities::{prelude::*, *};
use async_graphql::{dynamic::*, Value};
use jwt_simple::prelude::JWTClaims;
use sea_orm::ActiveValue;
use sea_orm::*;
use seaography::{CustomMutation, CustomMutationArgument};

use crate::config::global_config;
use crate::utilities::jwt::{
    ClaimRole,
    Role::{Guest, Owner},
};
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
                    Guest
                };
                let jwt = crate::utilities::jwt::create_jwt(id, role)?;
                Ok(Some(Value::from(jwt)))
            })
        }),
    }
}

pub fn mutation_comment() -> CustomMutation {
    CustomMutation {
        name: "comment".into(),
        arguments: vec![
            CustomMutationArgument {
                name: "content".into(),
                ty: TypeRef::named_nn(TypeRef::STRING),
            },
            CustomMutationArgument {
                name: "post_id".into(),
                ty: TypeRef::named_nn(TypeRef::INT),
            },
        ],
        ty: TypeRef::named_nn(TypeRef::STRING),
        resolver_fn: Box::new(|ctx| {
            FieldFuture::new(async move {
                let content = ctx.args.try_get("content")?.string()?.to_string();
                let post_id = ctx.args.try_get("post_id")?.i64()?;
                let sub = ctx
                    .data::<JWTClaims<ClaimRole>>()?
                    .subject
                    .clone()
                    .ok_or(async_graphql::Error::new(
                        "failed to convert sub".to_string(),
                    ))?
                    .parse::<i32>()?;
                let db = ctx.data::<sea_orm::DatabaseConnection>()?;

                let res = Comments::insert(comments::ActiveModel {
                    post_id: ActiveValue::Set(post_id.to_owned() as i32),
                    github_id: ActiveValue::Set(sub),
                    content: ActiveValue::Set(content),
                    ..Default::default()
                })
                .exec(db)
                .await?;
                Ok(Some(Value::from(res.last_insert_id)))
            })
        }),
    }
}
