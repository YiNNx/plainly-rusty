use seaography::{BuilderContext, FnGuard, GuardsConfig};
use std::collections::BTreeMap;

use super::guards::{all_block, user_query_only};

pub fn register_guard(context: BuilderContext) -> BuilderContext {
    let entity_guards = [("Posts".to_string(), Box::new(all_block))];
    let field_guards = [("Comments.content".to_string(), Box::new(user_query_only))];

    let mut m_entity: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in entity_guards {
        m_entity.insert(guard.0, guard.1);
    }
    let mut m_field: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in field_guards {
        m_field.insert(guard.0, guard.1);
    }
    BuilderContext {
        guards: GuardsConfig {
            entity_guards: m_entity,
            field_guards: m_field,
        },
        ..context
    }
}
