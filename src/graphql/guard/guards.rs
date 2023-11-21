use async_graphql::dynamic::ResolverContext;
use seaography::builder_context::guards;
use seaography::GuardAction;

use crate::graphql::index::{OperationType, OperationType::Query};
use crate::utilities::jwt::{verify_jwt, Role};

pub fn to_action(res: Result<guards::GuardAction, async_graphql::Error>) -> guards::GuardAction {
    match res {
        Ok(action) => action,
        Err(e) => GuardAction::Block(Some(e.message)),
    }
}

pub fn guard_public(ctx: &ResolverContext<'_>) -> guards::GuardAction {
    to_action(public(ctx))
}

pub fn guard_user_accessible(ctx: &ResolverContext<'_>) -> guards::GuardAction {
    to_action(user_accessible(ctx))
}

pub fn guard_user_editable(ctx: &ResolverContext<'_>) -> guards::GuardAction {
    to_action(user_editable(ctx))
}

pub fn guard_private(ctx: &ResolverContext<'_>) -> guards::GuardAction {
    to_action(private(ctx))
}

fn public(ctx: &ResolverContext<'_>) -> Result<guards::GuardAction, async_graphql::Error> {
    let ty = ctx.data::<OperationType>()?;

    if *ty == Query {
        Ok(GuardAction::Allow)
    } else {
        Ok(GuardAction::Block(Some("user query only".into())))
    }
}

fn user_accessible(ctx: &ResolverContext<'_>) -> Result<guards::GuardAction, async_graphql::Error> {
    let token = ctx.data::<String>()?;
    let ty = ctx.data::<OperationType>()?;
    let claims = verify_jwt(token)?;

    if claims.custom.rol == Role::Owner || *ty == Query {
        Ok(GuardAction::Allow)
    } else {
        Ok(GuardAction::Block(Some("user query only".into())))
    }
}

fn user_editable(ctx: &ResolverContext<'_>) -> Result<guards::GuardAction, async_graphql::Error> {
    let token = ctx.data::<String>()?;
    verify_jwt(token)?;

    Ok(GuardAction::Allow)
}

pub fn private(ctx: &ResolverContext<'_>) -> Result<guards::GuardAction, async_graphql::Error> {
    let token = ctx.data::<String>()?;
    let claims = verify_jwt(token)?;

    if claims.custom.rol == Role::Owner {
        Ok(GuardAction::Allow)
    } else {
        Ok(GuardAction::Block(Some("private field".into())))
    }
}
