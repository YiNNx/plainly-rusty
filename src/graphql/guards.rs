use async_graphql::dynamic::ResolverContext;
use seaography::{FnGuard, GuardAction};
use std::collections::BTreeMap;

use crate::config::global_config;
use crate::graphql::context::{OperationType, OperationType::Query};
use crate::utilities::jwt::Role;

fn public(ctx: &ResolverContext<'_>) -> Result<GuardAction, async_graphql::Error> {
    if *ctx.data::<OperationType>()? == Query {
        Ok(GuardAction::Allow)
    } else {
        if *ctx.data::<Role>()? == Role::Owner {
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