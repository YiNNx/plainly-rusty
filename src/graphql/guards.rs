use async_graphql::dynamic::ResolverContext;
use jwt_simple::prelude::JWTClaims;
use seaography::{FnGuard, GuardAction};
use std::collections::BTreeMap;

use crate::config::global_config;
use crate::graphql::context::{OperationType, OperationType::Query};
use crate::utilities::jwt::{ClaimRole, Role};

fn public(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    let op = ctx.data::<OperationType>()?;
    if *op == Query {
        Ok(GuardAction::Allow)
    } else {
        let claims = ctx.data::<JWTClaims<ClaimRole>>()?;
        if claims.custom.rol == Role::Owner {
            return Ok(GuardAction::Allow);
        }
        Ok(GuardAction::Block(Some("no permission".into())))
    }
}

fn to_action(res: Result<GuardAction, async_graphql::Error>) -> GuardAction {
    match res {
        Ok(action) => action,
        Err(e) => GuardAction::Block(Some(if global_config().application.debug {
            format!("no permission [error: {}]", e.message)
        } else {
            "no permission".into()
        })),
    }
}

pub fn guard_public(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(public(ctx))
}

pub fn entity_guards(entity_guards: Vec<(&str, FnGuard)>) -> BTreeMap<String, FnGuard> {
    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in entity_guards {
        map_guards.insert(guard.0.into(), guard.1);
    }
    map_guards
}

pub fn field_guards(field_guards: Vec<(&str, FnGuard)>) -> BTreeMap<String, FnGuard> {
    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in field_guards {
        map_guards.insert(guard.0.into(), guard.1);
    }
    map_guards
}

// fn _user_editable(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
//     let op = ctx.data::<OperationType>()?;
//     if *op == Mutation {
//         ctx.data::<JWTClaims<ClaimRole>>()?;
//     }
//     Ok(GuardAction::Allow)
// }

// pub fn _private(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
//     let token = ctx.data::<String>()?;
//     let claims = verify_jwt(token)?;

//     if claims.custom.rol == Role::Owner {
//         Ok(GuardAction::Allow)
//     } else {
//         Ok(GuardAction::Block(Some("private field".into())))
//     }
// }

// pub fn _guard_user_editable(ctx: &ResolverContext<'_>) -> GuardAction {
//     to_action(_user_editable(ctx))
// }

// pub fn _guard_private(ctx: &ResolverContext<'_>) -> GuardAction {
//     to_action(_private(ctx))
// }
