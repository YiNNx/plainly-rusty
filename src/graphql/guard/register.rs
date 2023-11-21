use seaography::FnGuard;
use std::collections::BTreeMap;

use super::guards::guard_user_accessible;

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
