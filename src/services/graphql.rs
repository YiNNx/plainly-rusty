use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::dynamic::Schema;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::config::global_config;
use crate::services::context::set_context_data;

pub async fn index(
    schema: web::Data<Schema>,
    req: GraphQLRequest,
    req_http: HttpRequest,
) -> GraphQLResponse {
    let request = req.into_inner();
    schema
        .execute(set_context_data(request, &req_http))
        .await
        .into()
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(
            &global_config().graphql.endpoint,
        ))))
}
