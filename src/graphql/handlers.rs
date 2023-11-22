use actix_web::{web, HttpRequest, HttpResponse, Result};
use async_graphql::dynamic::Schema;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::context::{jwt_from_req, operation_type_from_req};
use crate::config::global_config;
use crate::utilities::jwt::verify_jwt;

pub async fn index(
    schema: web::Data<Schema>,
    req: GraphQLRequest,
    req_http: HttpRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();

    if let Ok(claims) = verify_jwt(&jwt_from_req(&req_http).unwrap_or("".into())) {
        request = request.data(claims);
    }

    let op = operation_type_from_req(&request);
    let query = request.query.clone();
    schema
        .execute(request.data(op).data(query))
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
