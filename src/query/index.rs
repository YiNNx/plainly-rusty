use actix_web::{http::header::HeaderMap, web, HttpRequest, HttpResponse, Result};
use async_graphql::dynamic::*;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext};

use super::guard::register_guard;
use crate::config::global_config;
use crate::entities::*;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext =register_guard(BuilderContext::default());
}

pub fn schema(database: DatabaseConnection) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(builder, [comments, post_tags, posts, tags, users,]);
    builder.register_enumeration::<sea_orm_active_enums::CommentStatus>();
    builder.register_enumeration::<sea_orm_active_enums::PostStatus>();
    builder.register_enumeration::<sea_orm_active_enums::UserRole>();
    let schema = builder.schema_builder();
    schema.data(database).finish()
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<()> {
    headers.get("Token");
    // .and_then()
    Some(())
}

pub async fn index(
    schema: web::Data<Schema>,
    req: GraphQLRequest,
    req_http: HttpRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    if let Some(token) = get_token_from_headers(req_http.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(
            &global_config().graphql.endpoint,
        ))))
}
