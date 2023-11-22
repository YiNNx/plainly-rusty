use jwt_simple::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref JWT_KEY: HS256Key = HS256Key::generate();
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Role {
    Guest,
    Owner,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimRole {
    pub rol: Role,
}

pub fn create_jwt(sub: usize, rol: Role) -> Result<String, jwt_simple::Error> {
    let claims = Claims::with_custom_claims(ClaimRole { rol: rol }, Duration::from_days(7))
        .with_subject(sub);
    JWT_KEY.authenticate(claims)
}

pub fn verify_jwt(token: &String) -> Result<JWTClaims<ClaimRole>, jwt_simple::Error> {
    JWT_KEY.verify_token::<ClaimRole>(token, None)
}
