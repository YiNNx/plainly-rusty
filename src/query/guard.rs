use async_graphql::dynamic::ResolverContext;
use seaography::builder_context::guards;
use seaography::{BuilderContext, FnGuard, GuardAction, GuardsConfig};
use std::collections::BTreeMap;

pub fn register_guard(context: BuilderContext) -> BuilderContext {
    let entity_guards = [("Posts".to_string(), Box::new(guard_block))];
    let field_guards = [("Comments.content".to_string(), Box::new(guard_block))];

    let mut map_entity_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in entity_guards {
        map_entity_guards.insert(guard.0, guard.1);
    }
    let mut map_field_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in field_guards {
        map_field_guards.insert(guard.0, guard.1);
    }
    BuilderContext {
        guards: GuardsConfig {
            entity_guards: map_entity_guards,
            field_guards: map_field_guards,
        },
        ..context
    }
}

fn guard_block(_ctx: &ResolverContext<'_>) -> guards::GuardAction {
    GuardAction::Block(Some("blocked by guard_block".into()))
}
