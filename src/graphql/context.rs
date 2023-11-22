use crate::utilities::jwt::verify_jwt;
use actix_web::HttpRequest;

#[derive(PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
}

fn jwt_from_req(req: &HttpRequest) -> Option<String> {
    Some(
        req.headers()
            .get(actix_web::http::header::AUTHORIZATION)?
            .to_str()
            .unwrap_or("")
            .strip_prefix("Bearer ")
            .unwrap_or("")
            .into(),
    )
}

fn operation_type_from_req(req: &async_graphql::Request) -> OperationType {
    if req.query.contains("mutation") {
        OperationType::Mutation
    } else {
        OperationType::Query
    }
}

pub fn set_context_data(
    mut req: async_graphql::Request,
    req_http: &HttpRequest,
) -> async_graphql::Request {
    if let Ok(claims) = verify_jwt(&jwt_from_req(&req_http).unwrap_or("".into())) {
        req = req.data(claims);
    }
    let op = operation_type_from_req(&req);
    let query = req.query.clone();
    req.data(op).data(query)
}
