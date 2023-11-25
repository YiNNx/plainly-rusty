use crate::utilities::jwt::verify_jwt;
use actix_web::HttpRequest;

use crate::utilities::jwt::Role;

#[derive(PartialEq)]
pub enum OperationType {
    Query,
    Mutation,
}

pub type Subject = String;

fn jwt(req: &HttpRequest) -> Option<String> {
    Some(
        req.headers()
            .get(actix_web::http::header::AUTHORIZATION)?
            .to_str()
            .ok()?
            .strip_prefix("Bearer ")?
            .into(),
    )
}

fn claims(jwt: String) -> Option<(Subject, Role)> {
    let claims = verify_jwt(&jwt).ok()?;
    let rol = claims.custom.rol;
    let sub = claims.subject?.parse::<Subject>().ok()?;
    Some((sub, rol))
}

fn operation_type(req: &async_graphql::Request) -> OperationType {
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
    if let Some(claims) = jwt(req_http).and_then(claims) {
        req = req.data(claims.0).data(claims.1);
    }
    let op = operation_type(&req);
    req.data(op)
}
