use jwt_simple::prelude::*;
use lazy_static::lazy_static;

use crate::config::global_config;

lazy_static! {
    static ref JWT_KEY: HS256Key = if global_config().application.debug {
        HS256Key::from_bytes("test".as_bytes())
    } else {
        HS256Key::generate()
    };
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Role {
    Guest,
    Owner,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimCustom {
    pub rol: Role,
}

pub fn create_jwt(sub: String, rol: Role) -> Result<String, jwt_simple::Error> {
    let claims = Claims::with_custom_claims(ClaimCustom { rol: rol }, Duration::from_days(7))
        .with_subject(sub);
    JWT_KEY.authenticate(claims)
}

pub fn verify_jwt(token: &String) -> Result<JWTClaims<ClaimCustom>, jwt_simple::Error> {
    JWT_KEY.verify_token::<ClaimCustom>(token, None)
}
