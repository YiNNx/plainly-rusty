use async_graphql::dynamic::ResolverContext;
use log::info;
use seaography::builder_context::guards;
use seaography::GuardAction;

use crate::utilities::jwt::{verify_jwt, Role};

pub fn user_query_only(ctx: &ResolverContext<'_>) -> guards::GuardAction {
    let block = GuardAction::Block(Some("invalid authorization".into()));
    let token = if let Ok(t) = ctx.data::<String>() {
        t
    } else {
        return block;
    };
    
    // let claims = if let Ok(c) = verify_jwt(token) {
    //     c
    // } else {
    //     return block;
    // };
    // if claims.custom.rol == Role::Owner {
    //     return GuardAction::Allow;
    // }
    if token != "test" {
        return block;
    }
    let mutation = if let Ok(q) = ctx.data::<bool>() {
        q
    } else {
        return block;
    };
    info!("mutation type: {}",mutation);
    if *mutation == false {
        return GuardAction::Allow;
    }
    block
}

pub fn user_block(ctx: &ResolverContext<'_>) -> guards::GuardAction {
    // let token = ctx.data::<String>().unwrap_or(&"".to_string());
    // info!("data: {}", token);
    GuardAction::Block(Some("blocked by guard_block".into()))
}

pub fn all_block(_ctx: &ResolverContext<'_>) -> guards::GuardAction {
    GuardAction::Block(Some("blocked by guard_block".into()))
}
