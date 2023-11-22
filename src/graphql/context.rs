use actix_web::HttpRequest;

#[derive(PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
}

pub fn jwt_from_req(req: &HttpRequest) -> Option<String> {
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

pub fn operation_type_from_req(req: &async_graphql::Request) -> OperationType {
    if req.query.contains("mutation") {
        OperationType::Mutation
    } else {
        OperationType::Query
    }
}
