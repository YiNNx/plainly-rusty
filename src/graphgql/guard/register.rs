use seaography::FnGuard;
use std::collections::BTreeMap;

use super::guards::{all_block, user_query_only};

pub fn entity_guards() -> BTreeMap<String, FnGuard> {
    let entity_guards = [("Posts".to_string(), Box::new(all_block))];

    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in entity_guards {
        map_guards.insert(guard.0, guard.1);
    }
    map_guards
}

pub fn field_guards() -> BTreeMap<String, FnGuard> {
    let field_guards = [("Comments.content".to_string(), Box::new(user_query_only))];

    let mut map_guards: BTreeMap<String, FnGuard> = BTreeMap::new();
    for guard in field_guards {
        map_guards.insert(guard.0, guard.1);
    }
    map_guards
}
