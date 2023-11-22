use async_graphql::dynamic::ResolverContext;
use seaography::{FnGuard, GuardAction};
use std::collections::BTreeMap;

use crate::graphql::context::{OperationType, OperationType::Query};
use crate::utilities::jwt::{verify_jwt, Role};

pub fn entity_guards() -> BTreeMap<String, FnGuard> {
    let entity_guards = [("Posts".to_string(), Box::new(guard_user_accessible))];

    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in entity_guards {
        map_guards.insert(guard.0, guard.1);
    }
    map_guards
}

pub fn field_guards() -> BTreeMap<String, FnGuard> {
    let field_guards = [(
        "Comments.content".to_string(),
        Box::new(guard_user_accessible),
    )];

    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in field_guards {
        map_guards.insert(guard.0, guard.1);
    }
    map_guards
}

pub fn guard_public(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(public(ctx))
}

pub fn guard_user_accessible(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(user_accessible(ctx))
}

pub fn guard_user_editable(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(user_editable(ctx))
}

pub fn guard_private(ctx: &ResolverContext<'_>) -> GuardAction {
    to_action(private(ctx))
}

fn public(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    let ty = ctx.data::<OperationType>()?;

    if *ty == Query {
        Ok(GuardAction::Allow)
    } else {
        Ok(GuardAction::Block(Some("query only".into())))
    }
}

fn user_accessible(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    let token = ctx.data::<String>()?;
    let ty = ctx.data::<OperationType>()?;
    let claims = verify_jwt(token)?;

    if claims.custom.rol == Role::Owner || *ty == Query {
        Ok(GuardAction::Allow)
    } else {
        Ok(GuardAction::Block(Some("user query only".into())))
    }
}

fn user_editable(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    let token = ctx.data::<String>()?;
    verify_jwt(token)?;

    Ok(GuardAction::Allow)
}

pub fn private(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
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
        Err(e) => GuardAction::Block(Some(e.message)),
    }
}
