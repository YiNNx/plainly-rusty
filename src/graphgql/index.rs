use actix_web::{http::header::HeaderMap, web, HttpRequest, HttpResponse, Result};
use async_graphql::dynamic::Schema;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::config::global_config;

fn get_jwt(headers: &HeaderMap) -> Option<String> {
    Some(
        headers
            .get(actix_web::http::header::AUTHORIZATION)?
            .to_str()
            .unwrap_or("")
            .strip_prefix("Bearer ")
            .unwrap_or("")
            .into(),
    )
}

#[derive(PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
}

pub async fn index(
    schema: web::Data<Schema>,
    req: GraphQLRequest,
    req_http: HttpRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    if let Some(token) = get_jwt(req_http.headers()) {
        request = request.data(token);
    }
    request = if request.query.starts_with("mutation") {
        request.data(OperationType::Mutation)
    } else {
        request.data(OperationType::Query)
    };
    schema.execute(request).await.into()
}

pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(
            &global_config().graphql.endpoint,
        ))))
}
