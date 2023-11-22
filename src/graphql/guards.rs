use async_graphql::dynamic::ResolverContext;
use jwt_simple::prelude::JWTClaims;
use seaography::{FnGuard, GuardAction};
use std::collections::BTreeMap;

use crate::config::global_config;
use crate::graphql::context::{OperationType, OperationType::Mutation, OperationType::Query};
use crate::utilities::jwt::{verify_jwt, ClaimRole, Role};

pub fn entity_guards() -> BTreeMap<String, FnGuard> {
    let entity_guards: Vec<(&str, FnGuard)> = vec![
        ("Posts", Box::new(guard_public)),
        ("Tags", Box::new(guard_public)),
        ("PostTags", Box::new(guard_public)),
        ("Comments", Box::new(guard_user_editable)),
    ];

    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in entity_guards {
        map_guards.insert(guard.0.into(), guard.1);
    }
    map_guards
}

pub fn field_guards() -> BTreeMap<String, FnGuard> {
    let field_guards: Vec<(&str, FnGuard)> = vec![
        ("Comments.content", Box::new(guard_public)),
        ("Comments.content", Box::new(guard_public)),
    ];

    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in field_guards {
        map_guards.insert(guard.0.into(), guard.1);
    }
    map_guards
}

pub fn guard_public(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(public(ctx))
}

pub fn guard_user_editable(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(user_editable(ctx))
}

pub fn _guard_private(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(_private(ctx))
}

fn public(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    let op = ctx.data::<OperationType>()?;
    if *op == Query {
        Ok(GuardAction::Allow)
    } else {
        let claims = ctx.data::<JWTClaims<ClaimRole>>()?;
        if claims.custom.rol == Role::Guest {
            return Ok(GuardAction::Block(Some("no permission".into())));
        }
        Ok(GuardAction::Allow)
    }
}

fn user_editable(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    let query = ctx.data::<String>()?;
    if query.contains("Batch") {
        return Ok(GuardAction::Block(Some("no permission".into())));
    }

    let op = ctx.data::<OperationType>()?;
    if *op == Mutation {
        ctx.data::<JWTClaims<ClaimRole>>()?;
    }
    Ok(GuardAction::Allow)
}

pub fn _private(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    let token = ctx.data::<String>()?;
    let claims = verify_jwt(token)?;

    if claims.custom.rol == Role::Owner {
        Ok(GuardAction::Allow)
    } else {
        Ok(GuardAction::Block(Some("private field".into())))
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
